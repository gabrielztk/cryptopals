#![allow(dead_code)]
use crate::set_1_basics::c_03_single_byte_xor_cipher::single_byte_xor_cipher_bytes;
use crate::set_1_basics::c_05_implement_repeating_key_xor::xor_cipher;
use crate::utils::hamming_distance::hamming_distance;

const KEYSIZES: [usize; 40] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
];

fn break_repeating_key_xor(cypher: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let key_size = find_keysize(cypher);
    let transposed = transpose(cypher, key_size);
    let key = solve_blocks(transposed, key_size);
    let decrypted = xor_cipher(cypher, &key);

    (decrypted, key)
}

fn score_key_size(bytes: &[u8], key_size: usize) -> f64 {
    let chunks: Vec<&[u8]> = bytes.chunks_exact(key_size).collect();

    if chunks.len() < 2 {
        return f64::MAX;
    }

    let mut distance_sum = 0;
    for idx in 1..chunks.len() {
        distance_sum += hamming_distance(chunks[idx - 1], chunks[idx]) / key_size as u32;
    }

    (distance_sum as f64) / (chunks.len() as f64)
}

fn find_keysize(bytes: &[u8]) -> usize {
    let mut best_score = f64::MAX;
    let mut best_key_size = 2;

    for key_size in KEYSIZES {
        let score = score_key_size(bytes, key_size);

        if score < best_score {
            best_score = score;
            best_key_size = key_size;
        }
    }

    best_key_size
}

fn transpose(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let chunks = bytes.chunks(key_size);
    let mut result = vec![Vec::with_capacity(bytes.len() / key_size); key_size];

    for chunk in chunks {
        chunk.iter().enumerate().for_each(|(idx, &byte)| {
            result[idx].push(byte);
        })
    }

    result
}

fn solve_block(block: &[u8]) -> u8 {
    let (_, _, key) = single_byte_xor_cipher_bytes(block);
    key
}

fn solve_blocks(blocks: Vec<Vec<u8>>, key_size: usize) -> Vec<u8> {
    (0..key_size).map(|idx| solve_block(&blocks[idx])).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use crate::utils::file::read_bytes;
        use base64::prelude::*;

        let bytes = read_bytes("files/set_1/6.txt");
        let decoded = BASE64_STANDARD.decode(bytes).unwrap();
        let (_, key) = break_repeating_key_xor(&decoded);

        assert_eq!(
            String::from_utf8_lossy(&key),
            "Terminator X: Bring the noise"
        );
    }
}
