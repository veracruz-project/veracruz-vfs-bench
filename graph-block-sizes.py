#!/usr/bin/env python3

import sys
import json
import re
import itertools as it

import matplotlib
matplotlib.use('SVG')
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.ticker import FuncFormatter
import numpy as np
import csv

MODES = [
    'read',
    'write',
    'update',
]

TYPES = [
    ('large-file', ''),
    ('incremental-file', 'incremental_'),
]

ORDERS = [
    'inorder',
    'reversed',
    'random'
]

# seaborn color palette
COLORS = [
    '#4c72b0',
    '#dd8452',
    '#55a868',
    '#c44e52',
    '#8172b3',
    '#937860',
    '#da8bc3',
    '#8c8c8c',
    '#ccb974',
    '#64b5cd'
]

# SI base2 prefixes
SI2 = [
    (1024**0, ''  ),
    (1024**1, 'Ki'),
    (1024**2, 'Mi'),
    (1024**3, 'Gi'),
    (1024**4, 'Ti'),
    (1024**5, 'Pi'),
    (1024**6, 'Ei'),
    (1024**7, 'Zi'),
    (1024**8, 'Yi'),
]

def throughput_si(x, pos=None):
    for scale, si in reversed(SI2):
        if x >= scale or scale == 1:
            return '%s %sB/s' % (
                re.sub(r'\.0*$', '', ('%.3f'%(x/scale))[:3]), si)

def size_si(x, pos=None):
    for scale, si in reversed(SI2):
        if x >= scale or scale == 1:
            return '%s %sB' % (
                re.sub(r'\.0*$', '', ('%.3f'%(x/scale))[:3]), si)

def main(graph_path, *results):
    # parse results
    result_map = []
    for result_tuple in results:
        name, path = result_tuple.split('=', 1)
        with open(path) as f:
            results = json.load(f)
            results = sorted(results, key=lambda x: x["block_size"])

        result_map.append((name, results))

    # construct the graph
    matplotlib.rc('font', family='sans-serif', size=11)
    matplotlib.rc('axes', titlesize='medium', labelsize='medium')
    matplotlib.rc('xtick', labelsize='small')
    matplotlib.rc('ytick', labelsize='small')

    gs = gridspec.GridSpec(nrows=len(TYPES)*len(ORDERS), ncols=len(MODES),
         wspace=0.25, hspace=0.35)

    fig = plt.figure(figsize=(len(MODES)*6, len(TYPES)*len(ORDERS)*3.5))

    for y, ((type_name, type_prefix), order) in enumerate(it.product(TYPES, ORDERS)):
        for x, mode in enumerate(MODES):
            ax = fig.add_subplot(gs[y, x])
            ax.text(0.5, 1.025,
                '%s %s %s' % (mode, type_name, order),
                ha='center',
                transform=ax.transAxes)

            for i, (engine, results) in enumerate(result_map):
                was_buffered = True
                for buffered in [True, False]:
                    name = '%s%s%s_%s' % (type_prefix, 'buffered_' if buffered else '', mode, order)
                    sizes = ['%09x' % r['block_size']
                        for r in results
                        if r['name'] == name]
                    throughputs = [float(r['size']) / float(r['runtime'])
                        for r in results
                        if r['name'] == name]

                    if len(sizes) == 0:
                        was_buffered = False
                        continue
                    elif not was_buffered:
                        buffered = True

                    # plot each measurement as points
                    ax.plot(
                        sizes,
                        throughputs,
                        '.',
                        color=COLORS[i], alpha=0.75)

                    unique_sizes = []
                    for s in sizes:
                        if s not in unique_sizes:
                            unique_sizes.append(s)
                    unique_throughputs = []
                    for s in unique_sizes:
                        unique_throughputs.append(
                            [t for s_t, t in zip(sizes, throughputs) if s_t == s])

                    mins = [min(ts) for ts in unique_throughputs]
                    maxs = [max(ts) for ts in unique_throughputs]
                    avgs = [sum(ts)/len(ts) for ts in unique_throughputs]

                    # plot average w/ errors
                    ax.plot(
                        [str(s) for s in unique_sizes],
                        avgs,
                        '-' if buffered else ':',
                        color=COLORS[i], alpha=0.75, label=engine + ('' if buffered else ' (no bufrw)'))
                    ax.fill_between(
                        [str(s) for s in unique_sizes],
                        mins,
                        maxs,
                        color=COLORS[i], alpha=0.25, linewidth=0)
     
            if x == len(MODES)-1:
                ax.legend(loc='upper left',
                    bbox_to_anchor=(1.025, 1.05))

            ax.yaxis.set_major_formatter(FuncFormatter(throughput_si))
            ax.set_ylim(0, None)
            #ax.set_xticks(unique_sizes)
            ax.set_xticklabels([size_si(int(s, 16)) for s in unique_sizes], rotation=270)
            ax.spines['right'].set_visible(False)
            ax.spines['top'].set_visible(False)

    fig.tight_layout()
    plt.savefig(graph_path, bbox_inches="tight")

if __name__ == "__main__":
    main(*sys.argv[1:])
