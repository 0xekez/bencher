#!/usr/bin/env python3

from typing import Dict
import random
import nftables

nft = nftables.Nftables()

def validateAndExecute(nftPayload):
    try:
        nft.json_validate(nftPayload)
    except Exception as e:
        print(e)
        return False
    rc, out, err = nft.json_cmd(nftPayload)
    if rc != 0:
        print(f"rc: {rc}, out: {out}, err: {err}")
        return False
    return True

def setupNFT(tableName="yeet", chainName="input"):
    nftPayload = { "nftables": [
        { "add": { "table": {
            "family": "ip",
            "name": tableName,
        } } },
        { "add": { "chain": {
            "family": "ip",
            "table": tableName,
            "name": chainName,
            "hook": "input",
        } } },
    ] }
    return validateAndExecute(nftPayload)

def teardownNFT(tableName="yeet", chainName="IN"):
    nftPayload = { "nftables": [
        { "flush": { "ruleset": None } }
    ] }
    return nft.json_cmd(nftPayload)

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
                "table": "yeet",
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

def jam(n):
    nftPayload = { "nftables": [] }
    for _ in range(n):
        address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        while address.startswith("192.168"):
            address = ".".join([str(random.randint(0, 255)) for _ in range(4)])
        prefixbits = random.randint(16, 32)
        action = random.choice(["drop", "accept"])
        nftPayload["nftables"].append(
            { "add": { "rule" : {
                "family": "ip",
                "table": "yeet",
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
    validateAndExecute(nftPayload)