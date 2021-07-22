function start_listen() {
    iperf3 -s -D
    qperf 2>&1 > /dev/null &
}

function stop_listen() {
    pkill iperf
    pkill qperf
}

case $1 in
    up)
        start_listen
	;;
    down)
        stop_listen
	;;
    *)
	echo "usage: $0 <up/down>"
	;;
esac
