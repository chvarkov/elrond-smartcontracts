erdpy --verbose contract deploy \
  --project=adder \
  --pem="./wallets/root.pem" \
  --gas-limit=80000000 \
  --proxy="https://testnet-gateway.elrond.com" \
  --outfile="adder/output/deploy-adder.json" \
  --recall-nonce --send
