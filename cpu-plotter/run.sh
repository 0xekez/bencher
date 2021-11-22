#!/bin/bash

function die() {
    echo "RIP In Peace"

    pkill iperf3
    pkill python3

    exit
}

case $1 in
    collect)
        # PERCENT=$(ps -A -o %cpu | awk '{s+=$1} END {print s}')
	# PERCENT=$(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1}')
	PERCENT=$(mpstat 1 1 | grep -A 5 "%idle" | tail -n 1 | awk -F " " '{print 100 -  $12}'a)
	echo "$PERCENT" >> $2
	;;
    plot)
	gnuplot -e "filename='$2'" usage.plot > $3 2> /dev/null
	;;
    up)
	DEGULARDATA="degular.csv"
	DEGULARPLOT="degular.svg"

	PRIVDATA="priv.csv"
	PRIVPLOT="priv.svg"

	TABLESDATA="tables.csv"
	TABLESPLOT="tables.svg"

	MTABLESDATA="many-tables.csv"
	MTABLESPLOT="many-tables.svg"

	MYPUBADDR="$2"
	SENDERPUBADDR="$3"
	MYPRIVADDR="$4"
	SENDPRIVADDR="$5"

	echo "starting up"
	iperf3 -s -D
	python3 -m http.server 80 &
	sleep 2

	trap 'die' ERR SIGINT SIGTERM

	while :
	do
	    # # This tries to make sure that each test has a test run
	    # # before them as some circumstantial evidence suggests
	    # # that matters.
	    # echo "running nothingburger test"
	    # echo $MYPUBADDR | nc -q 0 $SENDERPUBADDR 8657
	    # sleep 1
	    # for i in $(seq 0 8)
	    # do
	    # 	./$0 collect'nothing.csv' 'nothing.svg'
	    # 	./$0 plot 'nothing.csv' 'nothing.svg'
	    # 	sleep 1
	    # done
	    # sleep 30
	    
	    echo "running vpn test"
	    echo $MYPRIVADDR | nc -q 0 $SENDERPUBADDR 8657
	    sleep 1
	    # iperf tests take 10 seconds to run. Collect and plot are
	    # very cheap to execute. We conservatively say that
	    # they'll take 1/20th of a second to execute each on
	    # average and loop + `sleep 1` 8 times. This ought to
	    # place us comfortably inside of the test when we are
	    # measuring cpu usage.
	    for i in $(seq 0 8)
	    do
		./$0 collect $PRIVDATA $PRIVPLOT
		./$0 plot $PRIVDATA $PRIVPLOT
		sleep 1
	    done
	    # Give everyone time to settle down.
	    sleep 30
	    
	    echo "running regular test"
	    echo $MYPUBADDR | nc -q 0 $SENDERPUBADDR 8657
	    sleep 1
	    for i in $(seq 0 8)
	    do
		./$0 collect $DEGULARDATA $DEGULARPLOT
		./$0 plot $DEGULARDATA $DEGULARPLOT
		sleep 1
	    done
	    sleep 30

	    echo "running iptables test"
	    ./iptables.sh up $SENDERPUBADDR $SENDPRIVADDR
	    echo $MYPUBADDR | nc -q 0 $SENDERPUBADDR 8657
	    sleep 1
	    for i in $(seq 0 8)
	    do
		./$0 collect $TABLESDATA $TABLESPLOT
		./$0 plot $TABLESDATA $TABLESPLOT
		sleep 1
	    done
	    ./iptables.sh down
	    sleep 30

	    echo "running many rules test"
	    ./iptables.sh up $SENDERPUBADDR $SENDPRIVADDR
	    ./iptables.sh many-rules
	    echo $MYPUBADDR | nc -q 0 $SENDERPUBADDR 8657
	    sleep 1
	    for i in $(seq 0 8)
	    do
		./$0 collect $MTABLESDATA $MTABLESPLOT
		./$0 plot $MTABLESDATA $MTABLESPLOT
		sleep 1
	    done
	    ./iptables.sh down
	    sleep 30

	done

	die
	;;
    *)
	echo "usage: $0 <up|plot|collect>"
	;;
esac
