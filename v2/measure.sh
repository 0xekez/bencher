# Runs a qperf TCP latency test against TARGET. Prints the results in
# the forma `<unix timestamp>,<latency>`.
function latency() {
    local target=$1
    local time=$(date +%s)
    local res=$(qperf $target -ub tcp_lat \
		    | tail -1 \
		    | cut -d'=' -f 2 \
		    | xargs)
    echo "$time,$res"
}

# Runs an iperf throughput test against TARGET. Prints the results in
# the format `<unix timestamp>,<send bits/sec>,<recv bits/sec>`.
function throughput() {
    local target=$1
    local time=$(date +%s)
    local res=$(iperf3 -c $target -J | jq -r '.end | "\(.sum_sent.bits_per_second),\(.sum_received.bits_per_second)"')
    echo "$time,$res"
}

case $1 in
    l)
	latency $2
	;;
    t)
	throughput $2
	;;
    *)
	echo "usage: $0 <l/t> <target address>"
	;;
esac
