#![allow(dead_code)]
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;

const AES_CHUNK_SIZE: usize = 16;
const AES_KEY_SIZE: usize = 16;

pub fn aes_in_ecb_mode(cipher_text: &[u8], key: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key).to_owned();

    let mut blocks = cipher_text
        .chunks(AES_CHUNK_SIZE)
        .map(|chunk| GenericArray::from_slice(chunk).to_owned())
        .fold(vec![], |mut vec, chunk| {
            vec.push(chunk);
            vec
        });

    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);

    blocks.into_iter().flatten().collect()
}

#[cfg(test)]
mod test {
    use super::aes_in_ecb_mode;
    use crate::utils::encoding::base64;
    use crate::utils::file::read_bytes;

    #[test]
    fn it_works() {
        let bytes = read_bytes("files/set_1/7.txt");
        let decoded = base64::decode(&bytes);
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = aes_in_ecb_mode(&decoded, key);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );
    }
}
