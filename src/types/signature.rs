use crate::types::key::PublicKey;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub signature: String,
    pub pub_key: PublicKey,
    pub account_number: u64,
    pub sequence: u64,
}
