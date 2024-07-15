use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicDumpError {
    
}

#[derive(Error, Debug)]
pub enum NcmDecodeError {
    #[error("Failed to parse magic header")]
    InvalidHeader,
    #[error("Failed to parse key")]
    InvalidKey,
    #[error("Failed to decrypt")]
    CryptError,
    #[error("Failed to parse salt")]
    SaltError,
}

impl From<CryptError> for NcmDecodeError {
    fn from(value: CryptError) -> Self {
        NcmDecodeError::CryptError
    }
}

#[derive(Error, Debug)]
pub enum CryptError {
    #[error("Failed to decrypt Aes128")]
    AesDecryptError,
}