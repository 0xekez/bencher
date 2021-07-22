function setup_target() {
    local target=$1

    # Create our CSV files as needed.
    test ! -e ${target}_latency.csv && \
	echo "timestamp,latency" > ${target}_latency.csv
    test ! -e ${target}_throughput.csv && \
	echo "timestamp,send_throughput,recv_throughput" > ${target}_throughput.csv
}

function measure_target() {
    local target=$1
    echo "measure: ($target)"
    bash measure.sh l $target >> ${target}_latency.csv
    bash measure.sh t $target >> ${target}_throughput.csv
}

case $1 in
    up)
	shift
	echo "targets: ($@)"
	for target in "$@"
	do
	    setup_target $target
	done

	bash listen.sh up

	while true
	do
	    for target in "$@"
	    do
		measure_target $target
	    done
	    sleep 300
	done
	;;
    down)
	bash listen.sh down
	;;

    *)
	echo "usage: $0 <up/down>"
	;;
esac
