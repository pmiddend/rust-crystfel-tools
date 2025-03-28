#!/usr/bin/env bash

set -euo pipefail

gnuplot -e 'title="Bar title"' -e 'inputfile="input.dat"' -e 'inputcolor="#f18f1f"' uc-plot.gnuplot
