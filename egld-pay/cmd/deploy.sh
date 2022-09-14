erdpy --verbose contract deploy \
  --project=egld-pay/sc \
  --pem="./egld-pay/wallets/root.pem" \
  --gas-limit=80000000 \
  --proxy="https://testnet-gateway.elrond.com" \
  --outfile="egld-pay/cmd/out/egld-pay.json" \
  --recall-nonce --send
