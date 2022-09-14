erdpy --verbose contract deploy \
  --project=sc \
  --pem="./wallets/root.pem" \
  --gas-limit=800000000 \
  --proxy="https://testnet-gateway.elrond.com" \
  --outfile="cmd/out/blogchain.json" \
  --recall-nonce --send
