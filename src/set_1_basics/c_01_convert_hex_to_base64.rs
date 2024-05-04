#![allow(dead_code)]
use crate::utils::encoding::{base64, hex};

fn convert_hex_to_base64(string: String) -> String {
    let hex_bytes = string.bytes().collect::<Vec<u8>>();
    let bytes = hex::decode(&hex_bytes);
    let base64 = base64::encode(&bytes);

    String::from_utf8_lossy(&base64).to_string()
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
