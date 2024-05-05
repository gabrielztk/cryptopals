#![allow(dead_code)]
use crate::set_1_basics::c_08_detect_aes_in_ecb_mode::score_cipher_text;
use crate::set_2_block_crypto::c_09_implement_pkcs7_padding::pkcs7_padding;
use crate::utils::aes::{aes_cbc_encrypt, aes_ecb_encrypt};
use rand::Rng;

const AES_KEY_SIZE: usize = 16;

#[derive(Debug, PartialEq, Eq)]
enum Encryption {
    ECB,
    CBC,
}

fn generate_random_block() -> [u8; AES_KEY_SIZE] {
    let mut rng = rand::thread_rng();
    let mut key = [0; AES_KEY_SIZE];

    key.iter_mut().for_each(|byte| {
        *byte = rng.gen::<u8>();
    });

    key
}

fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = vec![0; size];

    key.iter_mut().for_each(|byte| {
        *byte = rng.gen::<u8>();
    });

    key.into()
}

fn random_encryption(mut data: Vec<u8>) -> (Encryption, Vec<u8>) {
    let key = generate_random_block();
    let mut rng = rand::thread_rng();

    let encryption = match rand::random() {
        true => Encryption::CBC,
        false => Encryption::ECB,
    };

    let mut left_bytes = generate_random_bytes(rng.gen_range(5..=10));
    let mut right_bytes = generate_random_bytes(rng.gen_range(5..=10));

    let mut data_with_random_bytes =
        Vec::with_capacity(left_bytes.len() + data.len() + right_bytes.len());

    data_with_random_bytes.append(&mut left_bytes);
    data_with_random_bytes.append(&mut data);
    data_with_random_bytes.append(&mut right_bytes);

    let paded_data = pkcs7_padding(data_with_random_bytes);

    let cipher_text = match encryption {
        Encryption::CBC => {
            let initialization_vector = generate_random_block();
            aes_cbc_encrypt(&paded_data, &key, initialization_vector)
        }
        Encryption::ECB => aes_ecb_encrypt(&paded_data, &key),
    };

    (encryption, cipher_text)
}

fn detect_aes_mode(data: &[u8]) -> Encryption {
    if score_cipher_text(data) == 0 {
        Encryption::CBC
    } else {
        Encryption::ECB
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::encoding::hex;

    #[test]
    fn it_works() {
        let data = hex::decode("971ACD01C9C7ADEACC83257926F490FF".repeat(10).as_bytes());
        for _ in 0..100 {
            let (encription, encrypted_data) = random_encryption(data.clone());
            assert_eq!(encription, detect_aes_mode(&encrypted_data));
        }
    }
}
