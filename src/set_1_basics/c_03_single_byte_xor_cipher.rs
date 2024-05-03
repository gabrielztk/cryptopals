#![allow(dead_code)]
use crate::utils::character_frequency::score_byte_sequence;
use crate::utils::encoding::decode_hex;

pub fn single_byte_xor_cipher(cypher_text: String) -> String {
    let cypher_bytes = decode_hex(cypher_text.as_bytes());

    let (_, text_bytes, _) = single_byte_xor_cipher_bytes(&cypher_bytes);
    String::from_utf8_lossy(&text_bytes).to_string()
}

pub fn single_byte_xor_cipher_bytes(cypher_bytes: &[u8]) -> (f64, Vec<u8>, u8) {
    let mut best_score = f64::MAX;
    let mut best_key = 0;
    let mut best_text = vec![0; cypher_bytes.len()];

    (0..=255).for_each(|byte| {
        let possible_text = xor_cipher(cypher_bytes, byte);
        let score = score_byte_sequence(&possible_text);
        if score < best_score {
            best_score = score;
            best_text = possible_text;
            best_key = byte;
        }
    });

    (best_score, best_text, best_key)
}

pub fn xor_cipher(cypher_bytes: &[u8], character: u8) -> Vec<u8> {
    cypher_bytes
        .into_iter()
        .map(|byte| byte ^ character)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            single_byte_xor_cipher(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()
            ),
            "Cooking MC's like a pound of bacon"
        );
    }
}
