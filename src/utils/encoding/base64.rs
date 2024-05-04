pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let bit_vec: Vec<bool> = convert_byte_vector_to_bit_vector(bytes, 8);
    let byte_vec: Vec<u8> = convert_bit_vector_to_byte_vector(&bit_vec, 6);

    byte_vec
        .into_iter()
        .map(|byte| convert_byte_to_base64(byte))
        .collect()
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let decoded: Vec<u8> = bytes
        .into_iter()
        .map(|&byte| convert_base64_to_byte(byte))
        .collect();
    let bit_vec: Vec<bool> = convert_byte_vector_to_bit_vector(&decoded, 6);
    let byte_vec: Vec<u8> = convert_bit_vector_to_byte_vector(&bit_vec, 8);

    byte_vec
}

fn convert_byte_vector_to_bit_vector(bytes: &[u8], bits: usize) -> Vec<bool> {
    let size = bytes.len();
    let bit_vec: Vec<bool> =
        bytes
            .into_iter()
            .fold(Vec::with_capacity(size * bits), |mut acc, &b| {
                let mut bits = convert_byte_to_bit_vector(b, bits);
                acc.append(&mut bits);
                acc
            });

    bit_vec
}

fn convert_byte_to_bit_vector(byte: u8, bits: usize) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::with_capacity(bits);

    for i in (0..bits).rev() {
        let mask = 1 << i;
        let bit = byte & mask;
        result.push(bit >> i == 1);
    }

    result
}

fn convert_bit_vector_to_byte_vector(bit_vec: &[bool], bits: usize) -> Vec<u8> {
    let mut byte_vec: Vec<u8> = Vec::with_capacity(bit_vec.len() / bits);

    for i in (0..bit_vec.len()).step_by(bits) {
        let mut byte = 0u8;
        for j in 0..bits {
            let bit = (bit_vec[i + j] as u8) << (j as i32 - (bits as i32 - 1)).abs();
            byte += bit;
        }
        byte_vec.push(byte);
    }

    byte_vec
}

pub fn convert_byte_to_base64(byte: u8) -> u8 {
    if byte <= 25 {
        65 + byte
    } else if byte >= 26 && byte <= 51 {
        97 + byte - 26
    } else if byte >= 52 && byte <= 61 {
        48 + byte - 52
    } else if byte == 62 {
        '+' as u8
    } else if byte == 63 {
        '/' as u8
    } else {
        panic!("({})", byte)
    }
}

pub fn convert_base64_to_byte(byte: u8) -> u8 {
    if byte >= 65 && byte <= 90 {
        byte - 65
    } else if byte >= 97 && byte <= 122 {
        byte - 97 + 26
    } else if byte >= 48 && byte <= 57 {
        byte - 48 + 52
    } else if byte == '+' as u8 {
        62
    } else if byte == '/' as u8 {
        63
    } else {
        panic!("({})", byte)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        use base64::prelude::*;
        use hex;

        let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let bytes_to_compare = BASE64_STANDARD.encode(&bytes);
        let bytes = encode(&bytes);

        assert_eq!(
            bytes
                .into_iter()
                .map(|byte| byte as char)
                .collect::<String>(),
            bytes_to_compare
        );
    }

    #[test]
    fn test_decode() {
        use base64::prelude::*;

        let bytes =
            decode(&"PEXODbk6a48oMbAY6DDZsuLbc0uR9cp9hQ0QQGATyyCESq2NSsvhx5zKlLtz".as_bytes());
        let bytes_to_compare = BASE64_STANDARD
            .decode("PEXODbk6a48oMbAY6DDZsuLbc0uR9cp9hQ0QQGATyyCESq2NSsvhx5zKlLtz".as_bytes())
            .unwrap();

        assert_eq!(bytes, bytes_to_compare);
    }

    #[test]
    fn test_convert_byte_to_base64() {
        assert_eq!(convert_byte_to_base64(0), 'A' as u8);
        assert_eq!(convert_byte_to_base64(26), 'a' as u8);
        assert_eq!(convert_byte_to_base64(52), '0' as u8);
        assert_eq!(convert_byte_to_base64(62), '+' as u8);
        assert_eq!(convert_byte_to_base64(63), '/' as u8);
    }

    #[test]
    fn test_convert_byte_to_bit_vector() {
        assert_eq!(convert_byte_to_bit_vector(0, 8), vec![false; 8]);
        assert_eq!(convert_byte_to_bit_vector(0, 6), vec![false; 6]);
        assert_eq!(convert_byte_to_bit_vector(255, 8), vec![true; 8]);
        assert_eq!(convert_byte_to_bit_vector(127, 6), vec![true; 6]);

        assert_eq!(
            convert_byte_to_bit_vector(5, 8),
            vec![false, false, false, false, false, true, false, true]
        );
        assert_eq!(
            convert_byte_to_bit_vector(10, 8),
            vec![false, false, false, false, true, false, true, false]
        );
        assert_eq!(
            convert_byte_to_bit_vector(5, 6),
            vec![false, false, false, true, false, true]
        );
        assert_eq!(
            convert_byte_to_bit_vector(10, 6),
            vec![false, false, true, false, true, false]
        );
    }

    #[test]
    fn test_convert_bit_vector_to_byte_vector() {
        assert_eq!(
            convert_bit_vector_to_byte_vector(&vec![false, false, false, false, false, false], 6),
            vec![0]
        );
        assert_eq!(
            convert_bit_vector_to_byte_vector(&vec![false, true, true, true, true, true], 6),
            vec![31]
        );
        assert_eq!(
            convert_bit_vector_to_byte_vector(
                &vec![true, true, true, true, true, true, true, true, true, true, true, true,],
                6
            ),
            vec![63, 63]
        );

        assert_eq!(
            convert_bit_vector_to_byte_vector(
                &vec![false, false, false, false, false, false, false, false],
                8
            ),
            vec![0]
        );
        assert_eq!(
            convert_bit_vector_to_byte_vector(
                &vec![false, false, false, true, true, true, true, true],
                8
            ),
            vec![31]
        );
        assert_eq!(
            convert_bit_vector_to_byte_vector(
                &vec![
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true,
                ],
                8
            ),
            vec![255, 255]
        );
    }
}
