#!/usr/bin/env python3

import sys
import json
import re

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

ORDERS = [
    'inorder'
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

def main(results_path, graph_path):
    # parse results
    with open(results_path) as f:
        results = json.load(f)
        results = sorted(results, key=lambda x: x["size"])

    # construct the graph
    matplotlib.rc('font', family='sans-serif', size=11)
    matplotlib.rc('axes', titlesize='medium', labelsize='medium')
    matplotlib.rc('xtick', labelsize='small')
    matplotlib.rc('ytick', labelsize='small')

    gs = gridspec.GridSpec(nrows=1, ncols=len(MODES),
         wspace=0.25, hspace=0.25)

    fig = plt.figure(figsize=(len(MODES)*6, 1*3.5))

    for x, mode in enumerate(MODES):
        ax = fig.add_subplot(gs[0, x])
        ax.text(0.5, 1.125, mode, ha='center',
            transform=ax.transAxes)

        for i, order in enumerate(ORDERS):
            name = '_'.join([mode, order])
            sizes = ['%09x' % r['size']
                for r in results
                if r['name'] == name]
            throughputs = [float(r['size']) / float(r['runtime'])
                for r in results
                if r['name'] == name]

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
                '-',
                color=COLORS[i], alpha=0.75, label=mode)
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
        ax.set_xticklabels([size_si(int(s, 16)) for s in unique_sizes])
        ax.spines['right'].set_visible(False)
        ax.spines['top'].set_visible(False)

    fig.tight_layout()
    plt.savefig(graph_path, bbox_inches="tight")

if __name__ == "__main__":
    main(*sys.argv[1:])
