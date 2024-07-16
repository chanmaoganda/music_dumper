use std::io::{Cursor, Read};

use crate::NcmRc4;

pub struct Audio {
    ncm_rc4: NcmRc4,
    reader: Cursor<Vec<u8>>,
}

impl Audio {
    pub fn new(ncm_rc4: NcmRc4, encrypted: Vec<u8>) -> Self {
        let reader = Cursor::new(encrypted);
        Self {
            ncm_rc4,
            reader,
        }
    }

    pub fn get_decrypted_audio(mut self) -> Vec<u8> {
        let mut audio = self.reader.into_inner();
        self.ncm_rc4.decrypt(&mut audio);
        audio
    }
}