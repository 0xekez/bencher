#!/bin/bash

case $1 in
    up)
	shift
	# Set default chain policies
	iptables -P INPUT DROP
	iptables -P FORWARD DROP
	iptables -P OUTPUT ACCEPT

	# Accept on localhost
	iptables -A INPUT -i lo -j ACCEPT
	iptables -A OUTPUT -o lo -j ACCEPT

	# Allow established sessions to receive traffic
	iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT

	# Allow ssh
	iptables -I INPUT -p tcp --dport 22 -j ACCEPT
	# Allow http
	iptables -I INPUT -p tcp --dport 80 -j ACCEPT

	for address in "$@"
	do
	    iptables -A INPUT -p tcp -s $address -j ACCEPT
	done	
	;;
    many-rules)
	for address in $(echo 135.1.{0..20}.{0..255})
	do
	    iptables -A INPUT -p tcp -s $address -j ACCEPT
	done
	;;
    down)
	iptables -P INPUT ACCEPT
	iptables -P FORWARD ACCEPT
	iptables -P OUTPUT ACCEPT
	iptables -t nat -F
	iptables -t mangle -F
	iptables -F
	iptables -X
    ;;
    *)
	echo "usage: $0 <up|down> <address list..>"
	;;
esac
