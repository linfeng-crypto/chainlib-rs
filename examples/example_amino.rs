use chainlib::client::Client;
use chainlib::constant::ACCOUNT_ADDRESS_PREFIX;
use chainlib::error::Error;
use chainlib::hd_wallet::mnemonic::Mnemonic;
use chainlib::key_service::private_key_service::PrivateKeyService;
use chainlib::key_service::KeyService;
use chainlib::message::Transfer;
use chainlib::tx_builder::TxBuilder;
use chainlib::types::basic::{Amount, Denom, SyncMode};
use stdtx::Address;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let base_api_url = "http://127.0.0.1:1317".to_string();
    let client = Client::new(base_api_url);

    let fee = Amount::new(100000, Denom::Basecro);
    let gas = Some(300000);
    let memo = None;
    let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
    let mnemonic = Mnemonic::from_str(words, None).unwrap();
    let key_service = PrivateKeyService::new_from_mnemonic(mnemonic).unwrap();
    // or you can use ledger
    // let key_service = LedgerServiceHID::new(ACCOUNT_ADDRESS_PREFIX.to_string(), FUNDRAISER_PATH, false)
    //     .await
    //     .unwrap();
    let chain_id = "test".to_string();
    let mut builder = TxBuilder::new(key_service, chain_id, memo, Some(fee.clone()), gas);
    let (_, to_address) =
        Address::from_bech32("cro1s2gsnugjhpzac8m7necv3527jp28z9w002najd").unwrap();
    let from_address = builder.key_service.address().unwrap();
    let amount = Amount::new(100000000, Denom::Basecro);
    let msg = Transfer::new(from_address, to_address, amount);
    let address_str = from_address.to_bech32(ACCOUNT_ADDRESS_PREFIX);
    let (account_number, sequence) = client.get_account_info(&address_str).await?;
    builder
        .add_message(msg)
        .set_account_number(account_number)
        .set_sequence(sequence);
    let tx = builder.build(SyncMode::Sync).await.unwrap();
    let response = client.broadcast_tx(tx).await?;
    println!("{:?}", response);
    Ok(())
}
