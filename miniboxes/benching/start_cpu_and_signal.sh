#!/bin/bash
mpstat 1 >> $1 &
ssh 192.168.200.172 "qperf -t 60 192.168.200.171 tcp_bw" >> $2
ssh 192.168.200.172 "qperf -t 60 192.168.200.171 tcp_lat" >> $3
pkill mpstat