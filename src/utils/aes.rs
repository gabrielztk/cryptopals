#![allow(dead_code)]
use crate::set_1_basics::c_05_implement_repeating_key_xor::xor_cipher;
use aes::cipher::BlockEncrypt;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;

const AES_CHUNK_SIZE: usize = 16;
const AES_KEY_SIZES: [usize; 3] = [16, 24, 32];

pub fn aes_ecb_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.chunks(AES_CHUNK_SIZE)
        .map(|block| encrypt_block(block, key))
        .flatten()
        .collect()
}

pub fn aes_ecb_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.chunks(AES_CHUNK_SIZE)
        .map(|block| decrypt_block(block, key))
        .flatten()
        .collect()
}

pub fn aes_cbc_encrypt(
    data: &[u8],
    key: &[u8],
    initialization_vector: [u8; AES_CHUNK_SIZE],
) -> Vec<u8> {
    let mut initialization_vector = initialization_vector;

    data.chunks(AES_CHUNK_SIZE)
        .map(|block| {
            let block = xor_cipher(block, &initialization_vector);
            let ciphertext = encrypt_block(&block, key);
            initialization_vector = ciphertext;

            ciphertext
        })
        .flatten()
        .collect()
}

pub fn aes_cbc_decrypt(
    data: &[u8],
    key: &[u8],
    initialization_vector: [u8; AES_CHUNK_SIZE],
) -> Vec<u8> {
    let mut initialization_vector = initialization_vector;

    data.chunks(AES_CHUNK_SIZE)
        .map(|block| {
            let data = decrypt_block(&block, key);
            let data = xor_cipher(&data, &initialization_vector);
            initialization_vector = block.try_into().unwrap();

            data
        })
        .flatten()
        .collect()
}

pub fn encrypt_block(block: &[u8], key: &[u8]) -> [u8; AES_CHUNK_SIZE] {
    if !AES_KEY_SIZES.contains(&key.len()) {
        panic!();
    }

    let key = GenericArray::from_slice(key).to_owned();
    let mut block = GenericArray::from_slice(block).to_owned();

    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);

    block.into()
}

pub fn decrypt_block(block: &[u8], key: &[u8]) -> [u8; AES_CHUNK_SIZE] {
    if !AES_KEY_SIZES.contains(&key.len()) {
        panic!();
    }

    let key = GenericArray::from_slice(key).to_owned();
    let mut block = GenericArray::from_slice(block).to_owned();

    let cipher = Aes128::new(&key);
    cipher.decrypt_block(&mut block);

    block.into()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::encoding::base64;
    use crate::utils::file::read_bytes;

    #[test]
    fn test_aes_ecb_decrypt() {
        let bytes = read_bytes("files/set_1/7.txt");
        let decoded = base64::decode(&bytes);
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = aes_ecb_decrypt(&decoded, key);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );
    }

    #[test]
    fn test_aes_ecb_encrypt() {
        let bytes = read_bytes("files/set_1/7.txt");
        let decoded = base64::decode(&bytes);
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = aes_ecb_decrypt(&decoded, key);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );

        let encrypted = aes_ecb_encrypt(&decrypted, key);

        assert_eq!(encrypted, decoded);
    }

    #[test]
    fn test_aes_cbc_decrypt() {
        let bytes = read_bytes("files/set_2/10.txt");
        let decoded = base64::decode(&bytes);
        let initialization_vector = [0; 16];
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = aes_cbc_decrypt(&decoded, key, initialization_vector);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );
    }

    #[test]
    fn test_aes_cbc_encrypt() {
        let bytes = read_bytes("files/set_2/10.txt");
        let decoded = base64::decode(&bytes);
        let initialization_vector = [0; 16];
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = aes_cbc_decrypt(&decoded, key, initialization_vector);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );

        let encrypted = aes_cbc_encrypt(&decrypted, key, initialization_vector);

        assert_eq!(encrypted, decoded);
    }
}
