#![allow(dead_code)]
use crate::utils::encoding::encode_hex;

fn implement_repeating_key_xor(message: &str, key: &str) -> String {
    let mut message: Vec<u8> = message.bytes().collect::<Vec<u8>>();

    let key = key.bytes().collect::<Vec<u8>>();

    message = xor_cipher(message, key);

    encode_hex(&message)
        .into_iter()
        .map(|byte| byte as char)
        .collect()
}

pub fn xor_cipher(message: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    message
        .into_iter()
        .zip(key.into_iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let message = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let implementation = implement_repeating_key_xor(message, key);

        let target_string = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(implementation, target_string);
    }
}
