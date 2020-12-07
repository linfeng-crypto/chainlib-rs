use crate::hd_wallet::mnemonic::MnemonicError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mnemonic error")]
    MnemonicError(#[from] MnemonicError),

    #[error("invalid input: {0}")]
    InputError(String),

    #[error("secp error")]
    SecpError(#[from] secp256k1::Error),

    #[error("serialize error: {0}")]
    SerializeError(String),
}
