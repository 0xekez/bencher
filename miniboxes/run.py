from tenant_fw import app, setupNFT

def main():
    ok = setupNFT()
    if not ok:
        return
    app.run(host="0.0.0.0", port="5001")

if __name__=="__main__":
    main()