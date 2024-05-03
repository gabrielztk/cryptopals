#![allow(dead_code)]
use super::c_03_single_byte_xor_cipher::single_byte_xor_cipher_bytes;
use crate::utils::encoding::decode_hex;

pub fn detect_single_character_xor(lines: Vec<String>) -> String {
    let lines: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|line| decode_hex(&line.bytes().collect::<Vec<u8>>()))
        .collect();

    let mut best_score = f64::MAX;
    let mut best_line = vec![];

    for line in lines {
        let (new_score, new_line) = single_byte_xor_cipher_bytes(line);
        if new_score < best_score {
            best_score = new_score;
            best_line = new_line;
        }
    }

    best_line.into_iter().map(|byte| byte as char).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn it_works() {
        let file = File::open("basic4.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        assert_eq!(
            detect_single_character_xor(lines),
            "Now that the party is jumping\n"
        );
    }
}
