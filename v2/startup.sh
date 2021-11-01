#!/bin/bash

sudo apt update
sudo apt -y install emacs jq git iperf3 qperf gnuplot
git clone https://github.com/ZekeMedley/bencher.git

# configure bencher.service

sudo systemctl start bencher
sudo systemctl enable bencher
