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
        } } },
        { "add": { "chain": {
            "family": "ip",
            "table": tableName,
            "name": chainName,
            "hook": "input",
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
    ret = []
    for _ in range(n):
        address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        while address.startswith("192.168"):
            address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        prefixbits = random.randint(16, 32)
        ret.append(
            { "add": { "element": {
                "family": "ip",
                "table": "mytable",
                "name": "whitelist",
                "elem": f"{address}/{prefixbits}",
            } } },
        )

def jam(n):
    nftPayload = { "nftables": [] }
    validateAndExecute(nftPayload)



def run(testname="test"):
    numRules = 0
    initial = psutil.virtual_memory()
    baselineMemoryUsage = initial.total - initial.available
    for n in [30000]:
        # install rules (non-blocking)


        # get memory (non-blocking)
        usedMemory = initial.total - psutil.virtual_memory().available
        # start server (non-blocking)
        # start cpu logging (non-blocking)
        # ssh to client (blocking)
        # server stops automatically
        # stop cpu logging
        subprocess.run(f"./start_iperf3_and_cpu.sh iperf3-{testname}-{n}.log cpu-{testname}-{n}.log", shell=True, capture_output=True)

        # stop and gather cpu log


# run()