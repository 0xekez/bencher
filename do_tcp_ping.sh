nmap -sS -Pn -n -p$2 $1 | rg latency | rg "[[:digit:]]+\\.[[:digit:]]+" --only-matching
