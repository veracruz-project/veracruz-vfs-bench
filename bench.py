#!/usr/bin/env python3

import subprocess as sp
import sys
import os
import glob
import json


RULE = """
.PHONY: %(target)s
%(target)s:
        $(strip ( \\
                cd target/vc-fee && \\
                vc-fee -de -x jit \\
                        --arg veracruz-vfs-bench \\
                        --arg %(mode)s \\
                        --arg %(size)s \\
                        --arg %(block_size)s \\
                        --arg %(run)s \\
                        -p programs/veracruz-vfs-bench.wasm \\
                        -i programs \\
                        -o scratch \\
                        -o results ) )
"""

MODES = [
    "write_inorder",
    "update_inorder",
    "read_inorder",
]

SIZES = [2**x for x in range(10, 33+1, 3)]
BLOCK_SIZE = 512
RUNS = 5

def main(results_path="target/results.json", jobs=1):
    # create makefile
    with open('target/bench.mk', 'w') as mk:
        targets = []
        for mode in MODES:
            for size in SIZES:
                for run in range(RUNS):
                    target = "bench_%s_%s_%s_%s" % (mode, size, BLOCK_SIZE, run)
                    targets.append(target)

                    mk.write(
                        RULE.replace(4*' ', '\t') % dict(
                            target=target,
                            mode=mode,
                            size=size,
                            block_size=BLOCK_SIZE,
                            run=run,
                        )
                    )

    # run benchmarks, note that make should pick up the MAKEFLAGS env variable
    # if we're called from a makefile
    sp.check_call(
        ['make', '-f', 'target/bench.mk', '-j%s' % jobs] + targets,
    )

    # aggregate results
    print("aggregating into %s..." % results_path)
    results = []
    for result in glob.glob("target/vc-fee/results/*.json"):
        with open(result) as f:
            result = json.load(f)
            results.append(result)

    results = sorted(results, key=lambda x: x["size"])
    with open(results_path, 'w') as f:
        json.dump(results, f)

    print("done!")


if __name__ == "__main__":
    main(*sys.argv[1:])
