use crate::types::key::PublicKeyWrap;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub signature: String,
    pub pub_key: PublicKeyWrap,
    pub account_number: u64,
    pub sequence: u64,
}
