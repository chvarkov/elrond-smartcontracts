erdpy --verbose contract deploy \
  --project=loan-data \
  --pem="./wallets/root.pem" \
  --gas-limit=68000000 \
  --proxy="https://testnet-gateway.elrond.com" \
  --outfile="loan-data/output/deploy-loan-data.json" \
  --recall-nonce --send
