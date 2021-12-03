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

BASELINE = 1

MODES = [
    'read',
    'write',
    'update',
]

TYPES = [
    ('large-file', ''),
#    ('incremental-file', 'incremental_'),
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

def percentage(x, pos=None):
    return '%.0f %%' % (x*100) #re.sub(r'\.0*$', '', ('%.3f'%x)[:3])

def main(graph_path, *results):
    # parse results
    result_map = []
    baseline_results = None
    for i, result_tuple in enumerate(results):
        name, path = result_tuple.split('=', 1)
        with open(path) as f:
            results = json.load(f)
            results = sorted(results, key=lambda x: x["block_size"])

        result_map.append((name, results))

        if i == BASELINE:
            baseline_results = results

    # construct the graph
    matplotlib.rc('font', family='sans-serif', size=11)
    matplotlib.rc('axes', titlesize='medium', labelsize='medium')
    matplotlib.rc('xtick', labelsize='small')
    matplotlib.rc('ytick', labelsize='small')

    gs = gridspec.GridSpec(nrows=1, ncols=1,
         wspace=0.25, hspace=0.35)

    fig = plt.figure(figsize=(1.5*6, 1*3.5))

    ax = fig.add_subplot(gs[0, 0])
    ax.text(0.5, 1.025,
        result_map[0][0],
        ha='center',
        transform=ax.transAxes)

    for i, (order, mode) in enumerate([
            ('inorder', 'read'),
            ('inorder', 'write'),
            ('random', 'read'),
            ('random', 'write')]):
        for engine, results in result_map:
            if engine == result_map[BASELINE][0]:
                continue

            was_buffered = True
            for buffered in [True, False]:
                if not buffered:
                    continue

                name = '%s%s_%s' % ('buffered_' if buffered else '', mode, order)
                sizes = ['%09x' % r['block_size']
                    for r in results
                    if r['name'] == name]
                throughputs = [float(r['size']) / float(r['runtime'])
                    for r in results
                    if r['name'] == name]
                baseline_throughputs = [float(r['size']) / float(r['runtime'])
                    for r in baseline_results
                    if r['name'] == name]
                relative_throughputs = [(t-bt)/bt
                    for t, bt in zip(throughputs, baseline_throughputs)]

                if len(sizes) == 0:
                    was_buffered = False
                    continue
                elif not was_buffered:
                    buffered = True

                # plot each measurement as points
                ax.plot(
                    sizes,
                    relative_throughputs,
                    '.',
                    color=COLORS[i], alpha=0.75)

                unique_sizes = []
                for s in sizes:
                    if s not in unique_sizes:
                        unique_sizes.append(s)
                unique_throughputs = []
                for s in unique_sizes:
                    unique_throughputs.append(
                        [t for s_t, t in zip(sizes, relative_throughputs) if s_t == s])

                mins = [min(ts) for ts in unique_throughputs]
                maxs = [max(ts) for ts in unique_throughputs]
                avgs = [sum(ts)/len(ts) for ts in unique_throughputs]

                # plot average w/ errors
                ax.plot(
                    [str(s) for s in unique_sizes],
                    avgs,
                    '-' if buffered else ':',
                    color=COLORS[i], alpha=0.75, label=("%s %s" % (mode, order)) + ('' if buffered else ' (no bufrw)'))
                ax.fill_between(
                    [str(s) for s in unique_sizes],
                    mins,
                    maxs,
                    color=COLORS[i], alpha=0.25, linewidth=0)

    ax.plot(
        [str(s) for s in unique_sizes],
        [0 for s in unique_sizes],
        ':',
        color="#000000", alpha=0.75, label=result_map[BASELINE][0])

    ax.legend(loc='upper left',
        bbox_to_anchor=(1.025, 1.05))

    ax.yaxis.set_major_formatter(FuncFormatter(percentage))
    #ax.set_ylim(0, None)
    #ax.set_xticks(unique_sizes)
    ax.set_xticklabels([size_si(int(s, 16)) for s in unique_sizes], rotation=270)
    ax.spines['right'].set_visible(False)
    ax.spines['top'].set_visible(False)

    fig.tight_layout()
    plt.savefig(graph_path, bbox_inches="tight")

if __name__ == "__main__":
    main(*sys.argv[1:])
