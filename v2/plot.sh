function plot_latency() {
    local filename=$1

    gnuplot -e "filename='$filename'" latency.gnuplot
}

function plot_throughput() {
    local filename=$1

    gnuplot -e "filename='$filename'" throughput.gnuplot
}

case $1 in
    latency)
	plot_latency $2
	;;
    throughput)
	plot_throughput $2
	;;
    *)
	echo "usage: $0 <latency/throughput> <filename>"
	;;
esac
