# Elrond smart contracts

Just playing with elrond smart contracts

### Smart contracts list
* adder
* egld-pay
* loan-data

### Build

```shell
$ bash <SMART_CONTRACT_NAME>/build.sh
```

### Test
```shell
$ cd ./<SMART_CONTRACT_NAME>/sc && cargo test
```

### Generate new wallet

```shell
$ bash ./wallets/new.sh <WALLET_NAME>
```

### Deploy testnet
Generate `root` wallet.

```shell
$ bash ./wallets/new.sh root
```

Use faucet to get xEGLD.

```shell
$ bash ./<SMART_CONTRACT_NAME>/deploy.sh
```
