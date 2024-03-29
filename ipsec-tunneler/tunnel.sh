#!/bin/bash

set -e
set -o pipefail

case $1 in
    up)
	shift
	if [ "$4" == "" ]; then
	    echo "usage: $0 <my address> <their address> <my private address> <their private address>"
	    exit 1
	fi

	MY_ADDR="$1"
	THEIR_ADDR="$2"

	MY_PRIVATE_ADDR="$3"
	THEIR_PRIVATE_ADDR="$4"

	FLOW_ID=42

	# These are completely arbitrary and made up
	AUTH_KEY=0x6e3a0ac98f3da89da0b95d73f1703ceb22620bab5830521502147aaab2a28d40
	ENC_KEY=0x6e3a0ac98f3da89da0b95d73f1703ceb22620bab5830521502147aaab2a28d40

	# States
	ip xfrm state add src $MY_ADDR dst $THEIR_ADDR proto esp spi $FLOW_ID \
	   reqid $FLOW_ID mode tunnel auth sha256 $AUTH_KEY enc aes $ENC_KEY
	ip xfrm state add src $THEIR_ADDR dst $MY_ADDR proto esp spi $FLOW_ID \
	   reqid $FLOW_ID mode tunnel auth sha256 $AUTH_KEY enc aes $ENC_KEY

	# And their corresponding policies
	ip xfrm policy add src $MY_PRIVATE_ADDR dst $THEIR_PRIVATE_ADDR dir out \
	   tmpl src $MY_ADDR dst $THEIR_ADDR proto esp reqid $FLOW_ID mode tunnel
	ip xfrm policy add src $THEIR_PRIVATE_ADDR dst $MY_PRIVATE_ADDR dir in \
	   tmpl src $THEIR_ADDR dst $MY_ADDR proto esp reqid $FLOW_ID mode tunnel

	ip addr add $MY_PRIVATE_ADDR dev lo
	ip route add $THEIR_PRIVATE_ADDR dev eth0 src $MY_PRIVATE_ADDR
    ;;
    down)
	# Make a number of assumptions here about other programs not
	# adding addresses and ipsec rules.

	ip xfrm state deleteall
	ip xfrm policy deleteall

	ip addr flush dev lo
	ip addr add 127.0.0.1 dev lo
	ip addr add ::1/128 dev lo
    ;;
    *)
	echo "usage: $0 <up|down>"
    ;;
esac
