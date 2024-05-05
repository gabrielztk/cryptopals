#![allow(dead_code)]
use crate::set_1_basics::c_05_implement_repeating_key_xor::xor_cipher;
use crate::utils::aes::decrypt_block;

const AES_CHUNK_SIZE: usize = 16;

fn cbc_mode(data: &[u8], key: &[u8], initialization_vector: [u8; AES_CHUNK_SIZE]) -> Vec<u8> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::encoding::base64;
    use crate::utils::file::read_bytes;

    #[test]
    fn it_works() {
        let bytes = read_bytes("files/set_2/10.txt");
        let decoded = base64::decode(&bytes);
        let initialization_vector = [0; 16];
        let key = "YELLOW SUBMARINE".as_bytes();
        let decrypted = cbc_mode(&decoded, key, initialization_vector);

        assert!(
            String::from_utf8_lossy(&decrypted).contains("I'm back and I'm ringin' the bell \n")
        );
    }
}
