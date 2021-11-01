while true
do
    ADDRESS=$(nc -l 8657)
    iperf3 -c $ADDRESS --parallel 10
done
