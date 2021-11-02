from typing import Dict
from flask import Flask, request
import nftables

app = Flask(__name__)
nft = nftables.Nftables()

def setupNFT(tableName="yeet", chainName="IN"):
    nftPayload = { "nftables": [
        { "add": { "table": {
            "family": "inet",
            "name": tableName,
        } } },
        { "add": { "chain": {
            "family": "inet",
            "table": tableName,
            "name": chainName,
            "hook": "input",
        } } },
    ] }
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

def parseAddr(addr):
    pass

# send to this endpoint a JSON of the following format:
"""
{
    "method": str,
    "addr": str,
}
"""
# adds an ACCEPT rule for the address
@app.route("/", methods=["POST"])
def call():
    obj: Dict = request.json
    method = obj.get("method")
    addr = obj.get("addr")
    protocol, address, prefixbits = parseAddr(addr)
    # print(method, addr)
    if method == "add":
        nftPayload = { "nftables": [
            { "add": { "rule": {
                "family": "inet",
                "table": "yeet",
                "chain": "IN",
                "expr": [
                    { "match": {
                        "left": { "payload": {
                            "protocol": protocol,
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
        try:
            nft.json_validate(nftPayload)
        except Exception as e:
            print(f"validate{e}")
            return 'failure'
        rc, out, err = nft.json_cmd(nftPayload)
        if rc != 0:
            print(f"err: {err}")
            return 'failure'
        else:
            return 'success'
            
    elif method == "delete":
        pass
    