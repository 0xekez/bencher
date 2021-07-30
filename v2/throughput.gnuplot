set datafile separator comma

stats filename using 2

set xdata time
set timefmt "%s"
set key autotitle columnhead
set terminal svg
set logscale y 2

set label gprintf("Mean = %g", STATS_mean) at graph 0.1, graph 0.1 front
set label gprintf("Stddev = %g", STATS_stddev) at graph 0.1, graph 0.15 front

set style fill transparent solid 0.2 noborder

plot filename using 1:2 with lines, '' using 1:3 with lines, STATS_mean title " Mean" lw 2, STATS_lo_quartile with filledcurves y=STATS_up_quartile title " += 1 quartile"
