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

    pub fn get_decrypted_audio(&mut self) -> Vec<u8> {
        let mut audio = Vec::new();
        self.reader.read_to_end(&mut audio);
        audio
    }
}

impl Read for Audio {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.reader.read(buf)?;
        self.ncm_rc4.decrypt(buf);
        Ok(size)
    }
}

#[test]
fn chain() {
    let a = Cursor::new(vec![1, 2, 3]);
    let b = Cursor::new(vec![4, 5, 6]);
    let mut chain = a.chain(b);
    let mut buf = [0u8; 3];
    chain.read(&mut buf).unwrap();
    println!("{:?}", buf);
    buf = [0u8; 3];
    chain.read(&mut buf).unwrap();
    println!("{:?}", buf);
    buf = [0u8; 3];
    chain.read(&mut buf).unwrap();
    println!("{:?}", buf);
}