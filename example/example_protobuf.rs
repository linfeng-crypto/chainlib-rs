use anyhow::Error;

use cro_sign_tool::constant::ACCOUNT_ADDRESS_PREFIX;
use cro_sign_tool::hd_wallet::mnemonic::Mnemonic;
use cro_sign_tool::key_service::private_key_service::PrivateKeyService;
use cro_sign_tool::key_service::KeyService;
use cro_sign_tool::proto::cosmos::base::v1beta1::Coin;
use cro_sign_tool::proto::cosmos::tx::v1beta1::Fee;
use cro_sign_tool::proto::tendermint::rpc::grpc::{
    broadcast_api_client::BroadcastApiClient, RequestBroadcastTx, ResponseBroadcastTx,
};
use cro_sign_tool::tx_builder::grpc::TxBuilder;

struct Client {
    base_api_url: String,
    grpc_url: String,
}

impl Client {
    fn new(base_api_url: String, grpc_url: String) -> Self {
        Self {
            base_api_url,
            grpc_url,
        }
    }

    pub async fn get_account_info(&self, address: String) -> Result<(u64, u64), Error> {
        let url = format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            self.base_api_url, address
        );
        let response = reqwest::get(&url)
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .expect("get account info response error");
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

    pub async fn broadcast_tx(self, tx: Vec<u8>) -> ResponseBroadcastTx {
        let request = RequestBroadcastTx { tx };
        let mut client = BroadcastApiClient::connect(self.grpc_url)
            .await
            .expect("connect to grpc server failed");

        let request = tonic::Request::new(request.clone());
        let response = client.broadcast_tx(request).await.unwrap();
        let tx_response = response.into_inner();
        tx_response
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    // base api url is set as `address = "tcp://0.0.0.0:1317"` in $CHAIN_MAIND_HOME/config/app.toml
    let base_api_url = "http://127.0.0.1:1317".to_string();
    // grpc url is set in `grpc_laddr` in $CHAIN_MAIND_HOME/config/config.toml
    let grpc_url = "http://127.0.0.1:1234".to_string();
    let client = Client::new(base_api_url, grpc_url);

    let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
    let mnemonic = Mnemonic::from_str(words, None).unwrap();
    let key_service = PrivateKeyService::new_from_mnemonic(mnemonic).unwrap();
    let chain_id = "test".into();
    let fee = Fee {
        amount: vec![Coin {
            denom: "basecro".to_string(),
            amount: 10000.to_string(),
        }],
        gas_limit: 300000,
        payer: "".to_string(),
        granter: "".to_string(),
    };

    let timeout_height = 0;
    let mut builder = TxBuilder::new(key_service, chain_id, None, timeout_height, Some(fee));

    let self_address = builder.key_service.address().unwrap();
    let address_str = self_address.to_bech32(ACCOUNT_ADDRESS_PREFIX);

    // update account info
    let (account_number, sequence) = client.get_account_info(address_str).await.unwrap();

    // add msg
    let to_address = "cro1fj6jpmuykvra4kxrw0cp20e4vx4r8eda8q3yn9".into();
    let amount = Coin {
        denom: "basecro".into(),
        amount: 100000000.to_string(),
    };
    let msg = builder.create_msg(to_address, amount).unwrap();
    builder
        .add_message(msg)
        .set_account_number(account_number)
        .set_sequence(sequence);

    let b64_tx = builder.build().await.unwrap();
    let tx_bytes = base64::decode(b64_tx).unwrap();
    let response = client.broadcast_tx(tx_bytes).await;
    println!("grpc response: {:?}", response);
    Ok(())
}
