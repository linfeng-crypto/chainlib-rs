use crate::config::ACCOUNT_ADDRESS_PREFIX;
use bech32::{self, u5};
use serde::{Deserialize, Serialize};

/// Size in bytes of a 256-bit hash
pub const HASH_SIZE_ADDRESS: usize = 32;
/// 32-byte for keys or hashes etc.
pub type H256 = [u8; HASH_SIZE_ADDRESS];

/// errors with bech32 transfer addresses
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CroAddressError {
    /// wrong prefix
    InvalidPrefix,
    /// problems when converting from text?
    ConvertError,
    /// bech32 errors
    Bech32Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(H256);

impl Address {
    pub fn to_cro(&self) -> Result<String, CroAddressError> {
        let checked_data: Result<Vec<u5>, _> = self
            .0
            .iter()
            .map(|u8| bech32::u5::try_from_u8(*u8))
            .collect();
        let checked_data = checked_data.map_err(|e| {
            let s = e.to_string();
            CroAddressError::Bech32Error(s)
        })?;
        bech32::encode(ACCOUNT_ADDRESS_PREFIX, checked_data)
            .map_err(|e| CroAddressError::Bech32Error(e.to_string()))
    }

    pub fn from_cro(encoded_addr: &str) -> Result<Self, CroAddressError> {
        if !encoded_addr.starts_with(ACCOUNT_ADDRESS_PREFIX) {
            return Err(CroAddressError::InvalidPrefix);
        }

        bech32::decode(encoded_addr)
            .map_err(|e| CroAddressError::Bech32Error(e.to_string()))
            .and_then(|decoded| {
                let hash: Vec<u8> = decoded.1.iter().map(|u_5| u_5.to_u8()).collect();
                Ok(hash)
            })
            .map(|hash| {
                let mut tree_root_hash = [0 as u8; HASH_SIZE_ADDRESS];
                tree_root_hash.copy_from_slice(&hash.as_slice());
                Self(tree_root_hash)
            })
    }

    pub fn from_slice(raw: &[u8]) -> Result<Self, CroAddressError> {
        let mut bits = [0; HASH_SIZE_ADDRESS];
        if bits.len() != HASH_SIZE_ADDRESS {
            return Err(CroAddressError::ConvertError);
        }
        bits.copy_from_slice(raw);
        Ok(bits.into())
    }
}

impl From<H256> for Address {
    fn from(h256: H256) -> Self {
        Self(h256)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address() {
        let raw = vec![
            225, 64, 125, 164, 34, 185, 24, 168, 86, 50, 135, 191, 64, 67, 61, 9, 169, 101, 197, 10,
        ];
        let bits = bech32::convert_bits(&raw, 8, 5, true).unwrap();
        let address = Address::from_slice(&bits).unwrap();
        let address_str = "cro1u9q8mfpzhyv2s43js7l5qseapx5kt3g2rf7ppf";
        assert_eq!(address.to_cro().unwrap(), address_str);
        let address = Address::from_cro(address_str).unwrap();
        assert_eq!(address.to_cro().unwrap(), address_str);
    }
}
