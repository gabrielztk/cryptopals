#![allow(dead_code)]
use std::collections::HashSet;

fn detect_aes_in_ecb_mode(lines: &[Vec<u8>]) -> usize {
    lines
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            let score = score_cipher_text(line);
            (score, idx)
        })
        .max()
        .unwrap()
        .1
}

fn score_cipher_text(data: &[u8]) -> usize {
    let unique_sections: HashSet<[u8; 16]> = data
        .chunks_exact(16)
        .map(|vec| vec.try_into().unwrap())
        .collect();

    (data.len() / 16) - unique_sections.len()
}

#[cfg(test)]
mod test {
    use super::detect_aes_in_ecb_mode;
    use crate::utils::{encoding::decode_hex, file::read_lines};

    #[test]
    fn it_works() {
        let lines = read_lines("files/set_1/8.txt");
        let decoded = lines
            .into_iter()
            .map(|line| decode_hex(&line))
            .collect::<Vec<Vec<u8>>>();

        let line = detect_aes_in_ecb_mode(&decoded);
        assert_eq!(line, 132);
    }
}
