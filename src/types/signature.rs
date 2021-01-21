use crate::error::Error;
use crate::types::basic::Fee;
use crate::types::key::PublicKeyWrap;
use crate::utils::codec::serde_to_str;
use serde::Serialize;

/// Signature used in Tx
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub signature: String,
    pub pub_key: PublicKeyWrap,
    pub account_number: u64,
    pub sequence: u64,
}

#[derive(Serialize, Debug, Clone)]
pub struct SignDoc<M: Serialize> {
    #[serde(serialize_with = "serde_to_str")]
    pub account_number: u64,
    #[serde(serialize_with = "serde_to_str")]
    pub sequence: u64,
    pub chain_id: String,
    pub memo: String,
    pub fee: Fee,
    pub msgs: Vec<M>,
}

impl<M: Serialize> SignDoc<M> {
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_value(self).map_err(|e| Error::SerializeError(e.to_string()))?;
        let sign_str = sorted_json::to_json(&value)
            .replace("\n", "")
            .replace(" ", "");
        Ok(sign_str.into_bytes())
    }
}
