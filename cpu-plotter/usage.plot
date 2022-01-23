set terminal svg

n = 20
array A[n]

samples(x) = $0 > (n-1) ? n : int($0+1)
mod(x) = int(x) % n
avg_n(x) = (A[mod($0)+1]=x, (sum [i=1:samples($0)] A[i]) / samples($0))

stats filename

set label gprintf("Mean = %g", STATS_mean) at graph 0.1, graph 0.1 front
set label gprintf("Stddev = %g", STATS_stddev) at graph 0.1, graph 0.15 front

set style fill transparent solid 0.2 noborder

plot filename with lines title "cpu usage %" linecolor rgb "light-grey", \
     filename using 0:(avg_n($1)) with lines lw 3 title sprintf("%d point moving average", n)


