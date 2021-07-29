set datafile separator comma
set xdata time
set timefmt "%s"
set key autotitle columnhead
set terminal svg
set logscale y 2

plot filename using 1:2 with lines, '' using 1:3 with lines