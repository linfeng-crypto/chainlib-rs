use crate::config::CRO;
use crate::utils::codec::serde_to_str;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SyncMode {
    Sync,
    Async,
    Block,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Denom {
    Basecro,
    Cro,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Amount {
    denom: Denom,
    #[serde(serialize_with = "serde_to_str")]
    amount: u64,
}

impl Amount {
    pub fn new(amount: u64, denom: Denom) -> Self {
        let amount = match denom {
            Denom::Basecro => amount,
            Denom::Cro => amount * CRO,
        };
        Self {
            denom: Denom::Basecro,
            amount: amount,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Fee {
    #[serde(serialize_with = "serde_to_str")]
    pub gas: u64,
    pub amount: Vec<Amount>,
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            gas: 2000000,
            amount: vec![],
        }
    }
}
