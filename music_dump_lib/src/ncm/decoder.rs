use std::fs::File;
use std::io::Read;

use crate::error::NcmDecodeError;
use crate::crypt::{self, Rc4};

const HEADER_KEY: [u8; 16] = [ 0x68, 0x7A, 0x48, 0x52, 0x41, 0x6D, 0x73, 0x6F, 0x35, 0x6B, 0x49, 0x6E, 0x62, 0x61, 0x78, 0x57 ];

const INFO_KEY: [u8; 16] = [ 0x23, 0x31, 0x34, 0x6C, 0x6A, 0x6B, 0x5F, 0x21, 0x5C, 0x5D, 0x26, 0x30, 0x55, 0x3C, 0x27, 0x28 ];

pub struct NcmDecoder {
    reader: File,
}

impl NcmDecoder {
    pub fn new(reader: File) -> Self {
        NcmDecoder {
            reader,
        }
    }

    pub fn decode(&mut self) -> Result<(), NcmDecodeError> {
        self.parse_header()?;
        let rc4 = self.parse_rc4_handler()?;

        Ok(())
    }

    fn parse_header(&mut self) -> Result<(), NcmDecodeError> {
        let mut header_buffer = [0; 10];
        let header_size = self.reader.read(& mut header_buffer)
            .map_err(|_| NcmDecodeError::InvalidHeader).unwrap();
        if header_size != 10 {
            return Err(NcmDecodeError::InvalidHeader);
        }
        Ok(())
    }

    fn parse_rc4_handler(&mut self) -> Result<Rc4, NcmDecodeError> {
        let encrypted_key = self.get_encrypted_rc4_key()?;
        let key = self.decrypt_rc4_key(encrypted_key)?;
        Ok(Rc4::new(&key))
    }
}

/// private utils to decode
impl NcmDecoder {

    fn get_encrypted_rc4_key(&mut self) -> Result<Vec<u8>, NcmDecodeError> {
        let mut key_length = [0; 4];
        let key_bytes = self.reader.read(&mut key_length).map_err(|_| NcmDecodeError::InvalidKey)?;
        if key_bytes != 4 {
            return Err(NcmDecodeError::InvalidKey);
        }
        
        let key_length = u32::from_ne_bytes(key_length) as u64;

        let reader_ref = self.reader.by_ref();
        let mut chunk = reader_ref.take(key_length);
        let mut key = Vec::new();
        chunk.read_to_end(&mut key).map_err(|_| NcmDecodeError::InvalidKey)?;
        Ok(key)
    }

    fn decrypt_rc4_key(&self, mut encrypted_key: Vec<u8>) -> Result<Vec<u8>, NcmDecodeError> {
        encrypted_key.iter_mut()
            .for_each(|byte| *byte = *byte ^ 0x64);

        let aes_decrypted_key = crypt::aes128_decrypt(encrypted_key, &HEADER_KEY)?;

        let salt = aes_decrypted_key[..17]
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();
        if salt != String::from("neteasecloudmusic") {
            return Err(NcmDecodeError::SaltError)
        }

        Ok(aes_decrypted_key[17..].to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decrypt_test() -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open("../target/test/8bite-honest.ncm")?;
        let mut ncm_decoder = NcmDecoder {
            reader: file
        };
        ncm_decoder.parse_header()?;
        ncm_decoder.parse_rc4_handler()?;
        Ok(())
    }
}