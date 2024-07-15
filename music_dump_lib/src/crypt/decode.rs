use aes::Aes128;
use cipher::block_padding::Pkcs7;
use cipher::BlockDecryptMut;
use cipher::KeyInit;


use crate::error::CryptError;



pub fn aes128_decrypt(encrypted: Vec<u8>, key: &[u8; 16]) -> Result<Vec<u8>, CryptError> {
    let result = Aes128::new(key.into())
        .decrypt_padded_vec_mut::<Pkcs7>(&encrypted)
        .map_err(|_| CryptError::AesDecryptError)?;
    Ok(result)
}

fn rc4_expected(key: &[u8]) -> Vec<usize> {
    let mut last_byte = 0;
    let mut key_box = (0..256).collect::<Vec<usize>>();
    let mut offsets = (0..key.len()).cycle();
    for i in 0..256 {
        let offset = offsets.next().unwrap();
        let c = (key_box[i] + last_byte + key[offset] as usize) & 0xff;
        key_box.swap(i, c);
        last_byte = c;
    }
    key_box
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn rc4_test() {

    }
}