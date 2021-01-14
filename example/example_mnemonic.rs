use anyhow::Error;
use cro_sign_tool::constant::ACCOUNT_ADDRESS_PREFIX;
use cro_sign_tool::hd_wallet::mnemonic::Mnemonic;
use cro_sign_tool::key_service::private_key_service::PrivateKeyService;
use cro_sign_tool::key_service::KeyService;
use cro_sign_tool::tx_builder::amino::TransferBuilder;
use cro_sign_tool::types::basic::Amount;
use cro_sign_tool::types::basic::{Denom, SyncMode};
use cro_sign_tool::types::transaction::Transaction;
use stdtx::Address;

struct Signer<T: KeyService + Clone> {
    key_service: T,
    address: Address,
    chain_id: String,
    memo: Option<String>,
    base_api_url: String,
}

fn private_key_service(words: &str, password: Option<String>) -> Result<PrivateKeyService, Error> {
    let mnemonic = Mnemonic::from_str(words, password)?;
    let key_service = PrivateKeyService::new_from_mnemonic(mnemonic)?;
    Ok(key_service)
}

impl<T: KeyService + Clone> Signer<T> {
    async fn new(
        key_service: T,
        chain_id: String,
        memo: Option<String>,
        base_api_url: String,
    ) -> Result<Self, Error> {
        let address = key_service.address()?;
        Ok(Self {
            key_service,
            address,
            chain_id,
            memo,
            base_api_url,
        })
    }

    async fn get_transaction(
        &self,
        gas: Option<u64>,
        fee: Amount,
        to_address: &str,
        transfer_amount: u64,
        transfer_denom: Denom,
        account_number: u64,
        sequence: u64,
    ) -> Result<Transaction, Error> {
        let mut builder = TransferBuilder::new(
            fee,
            gas,
            self.memo.clone(),
            self.key_service.clone(),
            self.chain_id.clone(),
        );
        let (_, to_address) = Address::from_bech32(to_address).unwrap();
        builder
            .add_transfer(transfer_amount, transfer_denom, to_address)
            .await?;
        let transaction = builder
            .build(account_number, sequence, SyncMode::Sync)
            .await?;
        Ok(transaction)
    }

    pub async fn get_account_info(&self) -> Result<(u64, u64), Error> {
        let address_str = self.address.to_bech32(ACCOUNT_ADDRESS_PREFIX);
        let url = format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            self.base_api_url, address_str
        );
        let response = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;
        // {'account': {'@type': '/cosmos.auth.v1beta1.BaseAccount', 'address': 'cro1u9q8mfpzhyv2s43js7l5qseapx5kt3g2rf7ppf', 'pub_key': {'@type': '/cosmos.crypto.secp256k1.PubKey', 'key': 'AntL+UxMyJ9NZ9DGLp2v7a3dlSxiNXMaItyOXSRw8iYi'}, 'account_number': '9', 'sequence': '25'}}
        let account_number = response["account"]["account_number"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let sequence = response["account"]["sequence"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        Ok((account_number, sequence))
    }

    pub async fn broadcast(&self, tx: Transaction) -> Result<String, Error> {
        let url = format!("{}/txs", self.base_api_url);
        println!("broadcast url: {}", url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&tx)
            .send()
            .await?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(Error::msg(format!("send failed, response: {:?}", response)));
        }
        let response = response.json::<serde_json::Value>().await?;
        let tx_hash = response["txhash"].as_str().unwrap();
        Ok(tx_hash.into())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let fee = Amount::new(100000, Denom::Basecro);
    let gas = Some(300000);
    let memo = None;
    let chain_id = "test".to_string();
    let base_api_url = "http://127.0.0.1:1317".to_string();

    // private key service
    let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
    let service = private_key_service(words, None).unwrap();

    let signer = Signer::new(service, chain_id, memo, base_api_url)
        .await
        .unwrap();
    let to_address = "cro1lnznp25kv5zy9wsmrulkph0udynxv3jt42jfgy";
    let transfer_amount = 1;
    let transfer_denom = Denom::Cro;
    let (account_number, sequence) = signer.get_account_info().await.unwrap();
    let tx = signer
        .get_transaction(
            gas,
            fee,
            to_address,
            transfer_amount,
            transfer_denom,
            account_number,
            sequence,
        )
        .await
        .unwrap();
    let tx_str = serde_json::to_string(&tx).unwrap();
    println!("{}", tx_str);
    let tx_hash = signer.broadcast(tx).await.unwrap();
    println!("{:?}", tx_hash);
}
