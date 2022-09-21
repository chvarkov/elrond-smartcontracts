erdpy --verbose contract deploy \
  --project=egld-pay \
  --pem="./wallets/root.pem" \
  --gas-limit=2000000 \
  --proxy="https://testnet-gateway.elrond.com" \
  --outfile="adder/output/deploy-egld-pay.json" \
  --recall-nonce --send
