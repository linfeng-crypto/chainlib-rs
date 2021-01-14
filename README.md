cro-sign-tool is a lib to create hd wallet and sign [CRO](https://github.com/crypto-com/chain-main) transfer transaction offline

# prepare
Before test, we need to send some coin amount to the hd-wallet which associated with the mnemonic words.

1.transfer some coin to mnemonic words account, to get the hd address, you can use the following code
```
let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
let mnemonic = Mnemonic::from_str(words, password)?;
let key_service = PrivateKeyService::new_from_mnemonic(mnemonic)?;
let address = key_service.address()?;
let address_str = address.to_bech32("cro");
println!("{}", address_str);
```
or you can recover mnemonic to local storage:
`chain-maind keys add hd-wallet --keyring-backend test --recover`
and use `chain-maind keys list --keyring-backend test` to see the address

2.Transfer some coin amount to the hd address using `chain-maind`:
```
chain-maind tx bank send \
    ${from_address} \
    ${hd_address} \
    100cro \
    --keyring-backend test \
    --chain-id test \
    --sign-mode amino-json
```

3.There are some coin in the hd-wallet now, you can test to sign offline and send the signed transaction to chain api url, to see the detail, go to example.

# build
`cargo build --example ledger`
`cargo build --example mnemonic`
`cargo build --example protobuf --features=grpc`