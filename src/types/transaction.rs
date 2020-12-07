use crate::types::basic::{Amount, Denom, Fee, SyncMode};
use crate::types::signature::Signature;

use crate::config::ACCOUNT_ADDRESS_PREFIX;
use crate::error::Error;
use serde::Serialize;
use stdtx::Address;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TransferValue {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Amount>,
}

impl TransferValue {
    pub fn new(from_address: Address, to_address: Address, amount: Amount) -> Result<Self, Error> {
        Ok(Self {
            from_address: from_address.to_bech32(ACCOUNT_ADDRESS_PREFIX),
            to_address: to_address.to_bech32(ACCOUNT_ADDRESS_PREFIX),
            amount: vec![amount],
        })
    }
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub value: TransferValue,
}

impl Transfer {
    pub fn new(
        from_address: Address,
        to_address: Address,
        amount: u64,
        denom: Denom,
    ) -> Result<Self, Error> {
        let amount = Amount::new(amount, denom);
        let transfer_value = TransferValue::new(from_address.into(), to_address.into(), amount)?;
        Ok(Self {
            transfer_type: "cosmos-sdk/MsgSend".into(),
            value: transfer_value,
        })
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Tx {
    #[serde(rename = "msg")]
    pub messages: Vec<Transfer>,
    pub fee: Fee,
    pub memo: String,
    pub signatures: Vec<Signature>,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub tx: Tx,
    pub mode: SyncMode,
}
