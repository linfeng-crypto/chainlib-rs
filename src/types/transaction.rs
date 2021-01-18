use crate::types::basic::{Fee, SyncMode};
use crate::types::signature::Signature;
use serde::Serialize;

/// tx in transfer transaction
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Tx<M: Serialize> {
    #[serde(rename = "msg")]
    pub messages: Vec<M>,
    pub fee: Fee,
    pub memo: String,
    pub signatures: Vec<Signature>,
}

/// transfer transaction
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction<M: Serialize> {
    pub tx: Tx<M>,
    pub mode: SyncMode,
}
