#![allow(dead_code)]
use std::collections::HashMap;

use crate::set_2_block_crypto::c_09_implement_pkcs7_padding::pkcs7_padding;
use crate::set_2_block_crypto::c_11_an_ecb_cbc_detection_oracle::{detect_aes_mode, Encryption};
use crate::utils::aes::aes_ecb_encrypt;

const UNKNOWN_KEY: [u8; 16] = [
    89, 69, 76, 76, 79, 87, 32, 83, 85, 66, 77, 65, 82, 73, 78, 69,
];

const UNKNOWN_SECRET: [u8; 138] = [
    82, 111, 108, 108, 105, 110, 39, 32, 105, 110, 32, 109, 121, 32, 53, 46, 48, 10, 87, 105, 116,
    104, 32, 109, 121, 32, 114, 97, 103, 45, 116, 111, 112, 32, 100, 111, 119, 110, 32, 115, 111,
    32, 109, 121, 32, 104, 97, 105, 114, 32, 99, 97, 110, 32, 98, 108, 111, 119, 10, 84, 104, 101,
    32, 103, 105, 114, 108, 105, 101, 115, 32, 111, 110, 32, 115, 116, 97, 110, 100, 98, 121, 32,
    119, 97, 118, 105, 110, 103, 32, 106, 117, 115, 116, 32, 116, 111, 32, 115, 97, 121, 32, 104,
    105, 10, 68, 105, 100, 32, 121, 111, 117, 32, 115, 116, 111, 112, 63, 32, 78, 111, 44, 32, 73,
    32, 106, 117, 115, 116, 32, 100, 114, 111, 118, 101, 32, 98, 121, 10,
];

fn byte_at_a_time_ecb_decryption() -> Vec<u8> {
    let (block_size, secret_size) = block_and_secret_size();

    let repeated_bytes_max_size = block_size - 1;

    let mut secret: Vec<u8> = vec![0; secret_size];
    let mut current_byte: usize = 0;
    let mut know_bytes = Vec::with_capacity(block_size);
    let mut byte_dictionary: HashMap<Vec<u8>, u8> = HashMap::with_capacity(256);

    while current_byte < secret_size {
        let repeated_bytes_size = repeated_bytes_max_size - (current_byte % block_size);

        let current_block = (repeated_bytes_size + current_byte) / block_size;
        let block_index = current_block * block_size;
        let repeated_bytes = vec!['A' as u8; repeated_bytes_size];

        if current_byte < 16 {
            know_bytes.extend_from_slice(&repeated_bytes);
            know_bytes.extend_from_slice(&secret[..current_byte]);
        } else {
            know_bytes
                .extend_from_slice(&secret[current_byte - repeated_bytes_max_size..current_byte]);
        }

        know_bytes.push(0);
        let know_bytes_size = know_bytes.len();

        byte_dictionary = (0..=255)
            .map(|byte| {
                know_bytes[know_bytes_size - 1] = byte;

                let encrypted = encrypt_with_secrets(&know_bytes);
                let block = encrypted[..block_size].to_vec();

                (block, byte)
            })
            .collect();

        let true_block =
            encrypt_with_secrets(&repeated_bytes)[block_index..block_index + block_size].to_vec();
        let &true_byte = byte_dictionary.get(&true_block).unwrap();

        secret[current_byte] = true_byte;
        current_byte += 1;

        know_bytes.clear();
        byte_dictionary.clear();
    }

    secret
}

fn encrypt_with_secrets(data: &[u8]) -> Vec<u8> {
    let mut new_data = vec![];
    new_data.extend_from_slice(data);
    new_data.extend_from_slice(&UNKNOWN_SECRET);
    let padded_data = pkcs7_padding(new_data);

    aes_ecb_encrypt(&padded_data, &UNKNOWN_KEY)
}

fn block_and_secret_size() -> (usize, usize) {
    let mut text = "".to_string();
    let mut padding_size = 0;
    let starting_block_size = encrypt_with_secrets(text.as_bytes()).len();

    loop {
        let new_block_size = encrypt_with_secrets(text.as_bytes()).len();
        if new_block_size != starting_block_size {
            return (
                new_block_size - starting_block_size,
                starting_block_size - padding_size,
            );
        }
        padding_size += 1;
        text += "A";
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_byte_at_a_time_ecb_decryption() {
        assert_eq!(byte_at_a_time_ecb_decryption(), UNKNOWN_SECRET);
    }

    #[test]
    fn test_block_size() {
        assert_eq!(block_and_secret_size(), (16, UNKNOWN_SECRET.len()));
    }
}
