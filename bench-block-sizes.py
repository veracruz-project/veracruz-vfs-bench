#!/usr/bin/env python3

import subprocess as sp
import sys
import os
import glob
import json


VC_FEE_RULE = """
.PHONY: %(target)s
%(target)s:
        $(strip ( \\
                cd target && \\
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
        rm -rf target/scratch/*
"""

WASMTIME_RULE = """
.PHONY: %(target)s
%(target)s:
        $(strip ( \\
                cd target && \\
                wasmtime \\
                        --mapdir /scratch::./scratch \\
                        --mapdir /results::./results \\
                        programs/veracruz-vfs-bench.wasm \\
                        %(mode)s \\
                        %(size)s \\
                        %(block_size)s \\
                        %(run)s ) )
        rm -rf target/scratch/*
"""


MODES = [
    "write_inorder",
    "update_inorder",
    "read_inorder",
    "write_reversed",
    "update_reversed",
    "read_reversed",
    "write_random",
    "update_random",
    "read_random",
    "buffered_write_inorder",
    "buffered_update_inorder",
    "buffered_read_inorder",
    "buffered_write_reversed",
    "buffered_update_reversed",
    "buffered_read_reversed",
    "buffered_write_random",
    "buffered_update_random",
    "buffered_read_random",
#    "incremental_write_inorder",
#    "incremental_update_inorder",
#    "incremental_read_inorder",
#    "incremental_write_reversed",
#    "incremental_update_reversed",
#    "incremental_read_reversed",
#    "incremental_write_random",
#    "incremental_update_random",
#    "incremental_read_random",
]

SIZE = 2**26
BLOCK_SIZES = [2**x for x in range(2, 26+1, 2)]
#BLOCK_SIZES = [2**x for x in range(0, 28+1, 2)]

RUNS = 5

def main(engine="vc-fee", results_path="target/results.json", jobs=1):
    if engine == "vc-fee":
        rule = VC_FEE_RULE
    elif engine == "wasmtime":
        rule = WASMTIME_RULE
    else:
        print("unknown engine %s?" % engine)
        return 1

    # create makefile
    with open('target/bench.mk', 'w') as mk:
        targets = []
        for mode in MODES:
            for size in [SIZE]:
                for block_size in BLOCK_SIZES:
                    for run in range(RUNS):
                        target = "bench_%s_%s_%s_%s" % (mode, size, block_size, run)
                        targets.append(target)

                        mk.write(
                            rule.replace(4*' ', '\t') % dict(
                                target=target,
                                mode=mode,
                                size=size,
                                block_size=block_size,
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
    for result in glob.glob("target/results/*.json"):
        with open(result) as f:
            result = json.load(f)
            results.append(result)

    results = sorted(results, key=lambda x: x["size"])
    with open(results_path, 'w') as f:
        json.dump(results, f)

    print("done!")


if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:]))
