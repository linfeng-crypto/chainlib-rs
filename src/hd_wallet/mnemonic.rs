use crate::types::key::PrivateKey;

use crate::config::FUNDRAISER_PATH;
use anyhow::Error;
use bip39::{Language, MnemonicType, Seed};
use hdwallet::{ChainPath, KeyChain};
use hdwallet::{DefaultKeyChain, ExtendedPrivKey};

pub struct Mnemonic {
    inner_mnemonic: bip39::Mnemonic,
    password: Option<String>,
}

impl Mnemonic {
    /// create new Mnemonic
    pub fn new(word_count: u32, password: Option<String>) -> Result<Self, Error> {
        let flag = match word_count {
            12 => MnemonicType::Words12,
            15 => MnemonicType::Words15,
            18 => MnemonicType::Words18,
            21 => MnemonicType::Words21,
            24 => MnemonicType::Words24,
            _ => return Err(Error::msg("invalid words count")),
        };
        let mnemonic = bip39::Mnemonic::new(flag, Language::English);
        let m = Mnemonic {
            inner_mnemonic: mnemonic,
            password,
        };
        Ok(m)
    }

    /// Create Mnemonic from words in string literal
    pub fn from_str(words: &str, password: Option<String>) -> Result<Self, Error> {
        let mnemonic = bip39::Mnemonic::from_phrase(words, Language::English)?;
        let m = Mnemonic {
            inner_mnemonic: mnemonic,
            password,
        };
        Ok(m)
    }

    /// Generates private key
    pub fn private_key(&self) -> Result<PrivateKey, Error> {
        let chain_path = ChainPath::from(FUNDRAISER_PATH);
        let password = self.password.clone().unwrap_or_default();
        let seed = Seed::new(&self.inner_mnemonic, &password)
            .as_bytes()
            .to_vec();
        let master_key = ExtendedPrivKey::with_seed(&seed).map_err(|e| {
            let msg = format!("{:?}", e);
            Error::msg(msg)
        })?;
        let key_chain = DefaultKeyChain::new(master_key);

        let (extended_private_key, _) = key_chain.derive_private_key(chain_path).map_err(|e| {
            let msg = format!("{:?}", e);
            Error::msg(msg)
        })?;
        Ok(extended_private_key.into())
    }
}
