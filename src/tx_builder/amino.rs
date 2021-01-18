use crate::error::Error;
use crate::key_service::KeyService;
use crate::types::basic::{Amount, Denom, Fee, SyncMode};
use crate::types::signature::Signature;
use crate::types::transaction::{Transaction, Transfer, Tx};
use crate::utils::codec::serde_to_str;

use serde::Serialize;
use stdtx::Address;

pub struct TransferBuilder<T> {
    pub fee: Amount,
    pub gas: Option<u64>,
    pub memo: String,
    pub key_service: T,
    pub chain_id: String,
    pub signatures: Vec<Signature>,
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Debug, Clone)]
struct SignMsg {
    #[serde(serialize_with = "serde_to_str")]
    pub account_number: u64,
    #[serde(serialize_with = "serde_to_str")]
    pub sequence: u64,
    pub chain_id: String,
    pub memo: String,
    pub fee: Fee,
    pub msgs: Vec<Transfer>,
}

impl<T> TransferBuilder<T>
where
    T: KeyService,
{
    pub fn new(
        fee: Amount,
        gas: Option<u64>,
        memo: Option<String>,
        key_service: T,
        chain_id: String,
    ) -> Self {
        let memo = memo.unwrap_or_default();
        Self {
            fee,
            gas,
            memo,
            key_service,
            chain_id,
            signatures: vec![],
            transfers: vec![],
        }
    }

    pub async fn add_transfer(
        &mut self,
        amount: u64,
        denom: Denom,
        to_address: Address,
    ) -> Result<(), Error> {
        let from_address = self.key_service.address()?;
        let transfer = Transfer::new(from_address, to_address, amount, denom)?;
        self.transfers.push(transfer);
        Ok(())
    }

    #[inline]
    fn get_fee(&self) -> Fee {
        Fee {
            gas: self.gas.unwrap_or(20000),
            amount: vec![self.fee.clone()],
        }
    }

    async fn sign(&mut self, account_number: u64, sequence: u64) -> Result<(), Error> {
        let fee = self.get_fee();
        let sign_msg = SignMsg {
            account_number,
            sequence,
            chain_id: self.chain_id.clone(),
            memo: self.memo.clone(),
            fee,
            msgs: self.transfers.clone(),
        };
        let value =
            serde_json::to_value(&sign_msg).map_err(|e| Error::SerializeError(e.to_string()))?;
        let sign_str = sorted_json::to_json(&value)
            .replace("\n", "")
            .replace(" ", "");
        let signature = self.key_service.sign(sign_str.as_bytes()).await?;
        let public_key = self.key_service.public_key()?;

        let signature = Signature {
            signature,
            pub_key: public_key.into(),
            account_number,
            sequence,
        };
        self.signatures.push(signature);
        Ok(())
    }

    pub async fn build(
        &mut self,
        account_number: u64,
        sequence: u64,
        sync_mode: SyncMode,
    ) -> Result<Transaction, Error> {
        self.sign(account_number, sequence).await?;
        let fee = self.get_fee();
        let tx = Tx {
            messages: self.transfers.clone(),
            fee,
            memo: self.memo.clone(),
            signatures: self.signatures.clone(),
        };
        let transaction = Transaction {
            tx,
            mode: sync_mode,
        };
        Ok(transaction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constant::ACCOUNT_ADDRESS_PREFIX;
    use crate::hd_wallet::mnemonic::Mnemonic;
    use crate::key_service::private_key_service::PrivateKeyService;
    use crate::types::basic::Amount;
    use crate::types::key::PublicKey;
    use crate::types::transaction::TransferValue;

    #[tokio::test]
    async fn test_tx_builder() {
        let fee = Amount::new(100000, Denom::Basecro);
        let gas = Some(300000);
        let memo = None;
        let words = "dune car envelope chuckle elbow slight proud fury remove candy uphold puzzle call select sibling sport gadget please want vault glance verb damage gown";
        let mnemonic = Mnemonic::from_str(words, None).unwrap();
        let key_service = PrivateKeyService::new_from_mnemonic(mnemonic).unwrap();
        let chain_id = "test".to_string();
        let mut builder = TransferBuilder::new(fee.clone(), gas, memo, key_service, chain_id);
        let (_, to_address) =
            Address::from_bech32("cro1s2gsnugjhpzac8m7necv3527jp28z9w002najd").unwrap();
        builder
            .add_transfer(100000000, Denom::Basecro, to_address.clone())
            .await
            .unwrap();
        let account_number = 0;
        let sequence = 0;
        let transfer = builder
            .build(account_number, sequence, SyncMode::Sync)
            .await
            .unwrap();
        let transfer_expected = Transaction {
            tx: Tx {
                fee: Fee {
                    gas: 300000,
                    amount: vec![fee],
                },
                memo: "".into(),
                signatures: vec![
                    Signature {
                        signature: "xi3rvdsoZMXhWq7MlgAMXpoVIZ0kv7uB00OrSRS8wxwoZhojZ5uGZ4shobn3ztOev4M1k5WVcBvVd+zTvzRHCg==".into(),
                        pub_key: PublicKey::from_base64_str("AntL+UxMyJ9NZ9DGLp2v7a3dlSxiNXMaItyOXSRw8iYi").unwrap().into(),
                        account_number,
                        sequence,
                    }
                ],
                messages: vec![
                    Transfer {
                        transfer_type: "cosmos-sdk/MsgSend".into(),
                        value: TransferValue {
                            from_address: "cro1u9q8mfpzhyv2s43js7l5qseapx5kt3g2rf7ppf".into(),
                            to_address: to_address.to_bech32(ACCOUNT_ADDRESS_PREFIX),
                            amount: vec![Amount::new(100000000, Denom::Basecro)],
                        }
                    }
                ]
            },
            mode: SyncMode::Sync,
        };
        assert_eq!(transfer, transfer_expected);
    }
}
