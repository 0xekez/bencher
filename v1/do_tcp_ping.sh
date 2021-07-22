# Getting these commands to pipe together properly in Rust is a pain
# and I am lazy / don't have the patience to fight it so we just
# execute this shell script instead.

# Takes an ip as argument 1 and a port as argument 2 and runs nmap to
# calculate the latency over the tcp connection. Netmap calculates
# latency by sending a SYN packet as if it was initiating a tcp
# connection and timing how long the corresponding SYN/ACK packet
# takes to arrive.
#
# I've done some adhoc testing of this and the results seem to very
# closely match that of regular ping execution.

nmap -sS -Pn -n -p$2 $1 | \
    rg latency | \
    rg "[[:digit:]]+\\.[[:digit:]]+" --only-matching
