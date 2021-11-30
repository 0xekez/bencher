#!/usr/bin/env python3

from typing import Dict
import nftables
import subprocess, psutil, random, subprocess, traceback

nft = nftables.Nftables()

def validateAndExecute(nftPayload):
    try:
        nft.json_validate(nftPayload)
    except Exception as e:
        print(e)
        traceback.print_stack()
        return False
    rc, out, err = nft.json_cmd(nftPayload)
    if rc != 0:
        print(f"rc: {rc}, out: {out}, err: {err}")
        traceback.print_stack()
        return False
    return True

def setupNFT(tableName="mytable", chainName="input"):
    nftPayload = { "nftables": [
        { "add": { "table": {
            "family": "ip",
            "name": tableName,
        } } },
        { "add": { "set": {
            "family": "ip",
            "table": tableName,
            "name": "whitelist",
            "type": "ipv4_addr",
            # "policy"
            "flags": ["interval"],
            "elem": [{ "prefix": {
                "addr": "192.168.200.1",
                "len": 24,
            } },
            ],
        } } },
        { "add": { "chain": {
            "family": "ip",
            "table": tableName,
            "name": chainName,
            "hook": "input",
            "policy": "drop",
        } } },
        { "add": { "rule": {
            "family": "ip",
            "table": tableName,
            "chain": chainName,
            "expr": [
                { "match": {
                    "left": { "payload": {
                        "protocol": "ip",
                        "field": "saddr",
                    } },
                    "right": "@whitelist",
                    "op": "==",
                } },
                { "accept": None },
            ],
        } } },
    ] }
    return validateAndExecute(nftPayload)

def teardownNFT():
    return nft.cmd("flush ruleset")

# send to this endpoint a JSON of the following format:
"""
{
    "method": str,
    "addr": str,
}
"""
# adds an ACCEPT rule for the address
def call(method, address, prefixbits):
    # method = obj.get("method")
    # addr = obj.get("addr")
    # address, prefixbits = parseAddr(addr)
    # print(method, addr)
    if method == "add":
        nftPayload = { "nftables": [
            { "add": { "rule": {
                "family": "ip",
                "table": "mytable",
                "chain": "input",
                "expr": [
                    { "match": {
                        "left": { "payload": {
                            "protocol": "ip",
                            "field": "saddr",
                        } },
                        "right": { "prefix": {
                            "addr": address,
                            "len": prefixbits,
                        } },
                        "op": "==",
                    } },
                    { "drop": None },
                ],
            } } },
        ] }
        return validateAndExecute(nftPayload)
            
    elif method == "delete":
        pass

def generateRules(n):
    ret = []
    for _ in range(n):
        address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        while address.startswith("192.168"):
            address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        prefixbits = random.randint(16, 32)
        action = random.choice(["drop", "accept"])
        ret.append(
            { "add": { "rule" : {
                "family": "ip",
                "table": "mytable",
                "chain": "input",
                "expr": [
                    { "match": {
                        "left": { "payload": {
                            "protocol": "ip",
                            "field": "saddr",
                        } },
                        "right": { "prefix": {
                            "addr": address,
                            "len": prefixbits,
                        } },
                        "op": "==",
                    } },
                    { action: None },
                ],
            } } },
        )
    return ret

def generateSetElements(n):
    elems = []
    for _ in range(n):
        address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        while address.startswith("192.168"):
            address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        prefixbits = random.randint(16, 32)
        elems.append(
            { "prefix": {
                "addr": address,
                "len": prefixbits,
            } }
        )
    
    return { "add": { "element": {
        "family": "ip",
        "table": "mytable",
        "name": "whitelist",
        "elem": elems,
    } } }

def run(testname="test"):
    teardownNFT()
    setupNFT()
    numRules = 0
    initial = psutil.virtual_memory()
    baselineMemoryUsage = initial.total - initial.available
    outputFile = f"{testname}.log"
    for n in [3000]:
        # install rules (quick)
        nftPayload = { "nftables": generateSetElements(n - numRules)}
        validateAndExecute(nftPayload)
        numRules = n

        # get memory (quick)
        usedMemory = initial.total - psutil.virtual_memory().available

        # start cpu logging (non-blocking)
        # ssh to client (blocking)
        # server stops automatically
        # stop cpu logging
        cpuLog = f"cpu-{testname}-{n}.log"
        bwLog = f"qperfBw-{testname}-{n}.log"
        latLog = f"qperfLat-{testname}-{n}.log"
        subprocess.run(f"./start_cpu_and_signal.sh {cpuLog} {bwLog} {latLog}", shell=True)

        # gather cpu log

run()
