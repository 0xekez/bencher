function die() {
    echo "RIP In Peace"

    pkill iperf3
    pkill Python

    exit
}

case $1 in
    collect)
        PERCENT=$(ps -A -o %cpu | awk '{s+=$1} END {print s}')
	echo "$PERCENT" >> $2
	;;
    plot)
	gnuplot -e "filename='$2'" usage.plot > $3 2> /dev/null
	;;
    up)
	DATAFILE="data.csv"
	PLOTFILE="data.svg"

	MYADDR="$2"
	SENDERADDR="$3"

	iperf3 -s -D
	python3 -m http.server 80 &

	trap 'die' ERR SIGINT SIGTERM

	while :
	do
	    echo "running test"
	    echo $MYADDR | nc $SENDERADDR 8657
	    sleep 1
	    # iperf tests take 10 seconds to run. Collect and plot are
	    # very cheap to execute. We conservatively say that
	    # they'll take 1/20th of a second to execute each on
	    # average and loop + `sleep 1` 8 times. This ought to
	    # place us comfortably inside of the test when we are
	    # measuring cpu usage.
	    for i in $(seq 0 8)
	    do
		./$0 collect $DATAFILE $PLOTFILE
		./$0 plot $DATAFILE $PLOTFILE
		sleep 1
	    done
	    sleep 3
	done

	die
	;;
    *)
	echo "usage: $0 <up|plot|collect>"
	;;
esac
