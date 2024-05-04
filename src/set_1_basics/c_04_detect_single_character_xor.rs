#![allow(dead_code)]
use super::c_03_single_byte_xor_cipher::single_byte_xor_cipher_bytes;
use crate::utils::encoding::hex;

pub fn detect_single_character_xor(lines: &[Vec<u8>]) -> String {
    let decoded_lines: Vec<Vec<u8>> = lines.into_iter().map(|line| hex::decode(&line)).collect();

    let mut best_score = f64::MAX;
    let mut best_line = vec![];

    for line in decoded_lines {
        let (new_score, new_line, _) = single_byte_xor_cipher_bytes(&line);
        if new_score < best_score {
            best_score = new_score;
            best_line = new_line;
        }
    }

    String::from_utf8_lossy(&best_line).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::file::read_lines;

    #[test]
    fn it_works() {
        let lines = read_lines("files/set_1/4.txt");

        assert_eq!(
            detect_single_character_xor(&lines),
            "Now that the party is jumping\n"
        );
    }
}
