#![allow(dead_code)]
use crate::utils::encoding::{decode_hex, encode_64};

fn convert_hex_to_base64(string: String) -> String {
    let hex_bytes = string.bytes().collect::<Vec<u8>>();
    let bytes = decode_hex(&hex_bytes);
    let base64 = encode_64(&bytes);

    base64.into_iter().map(|byte| byte as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()), 
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
