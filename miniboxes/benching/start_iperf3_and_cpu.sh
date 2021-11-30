#!/bin/bash
iperf3 -s -1 >> $1 &
mpstat 1 >> $2 &
ssh 192.168.200.172 startflooding && pkill mpstat