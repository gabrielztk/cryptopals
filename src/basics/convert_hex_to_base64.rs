#![allow(dead_code)]

fn convert_hex_to_base64(string: String) -> String {
    let bytes = convert_hex_sequence_to_bytes_sequence(string);

    println!("{:?}", String::from_utf8(bytes.clone()));

    let bit_vec: Vec<bool> = convert_byte_vector_to_bit_vector(bytes);

    let hex_vec: Vec<u8> = convert_bit_vector_to_hex_byte_vector(bit_vec);

    hex_vec
        .into_iter()
        .map(|byte| convert_byte_to_base64(byte))
        .collect()
}

fn convert_hex_to_byte(most: u8, least: u8) -> u8 {
    fn map_to_byte(byte: u8) -> u8 {
        if byte >= 48 && byte <= 57 {
            byte - 48
        } else if byte >= 65 && byte <= 70 {
            byte - 55
        } else if byte >= 97 && byte <= 102 {
            return byte - 87;
        } else {
            panic!()
        }
    }

    map_to_byte(least) + (map_to_byte(most) << 4)
}

fn convert_byte_to_bit_vector(byte: u8) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::with_capacity(8);

    for i in (0..8).rev() {
        let mask = 1 << i;
        let bit = byte & mask;
        result.push(bit >> i == 1);
    }

    result
}

fn convert_byte_vector_to_bit_vector(bytes: Vec<u8>) -> Vec<bool> {
    let size = bytes.len();
    let bit_vec: Vec<bool> =
        bytes
            .into_iter()
            .fold(Vec::with_capacity(size * 8), |mut acc, b: u8| {
                let mut bits = convert_byte_to_bit_vector(b);
                acc.append(&mut bits);
                acc
            });

    bit_vec
}

fn convert_bit_vector_to_hex_byte_vector(bit_vec: Vec<bool>) -> Vec<u8> {
    let mut hex_vec: Vec<u8> = Vec::with_capacity(bit_vec.len() / 6);

    for i in (0..bit_vec.len()).step_by(6) {
        let mut byte = 0u8;
        for j in 0..6 {
            let bit = (bit_vec[i + j] as u8) << (j as i32 - 5).abs();
            byte += bit;
        }
        hex_vec.push(byte);
    }

    hex_vec
}

fn convert_hex_sequence_to_bytes_sequence(string: String) -> Vec<u8> {
    let string_as_numbers: Vec<u8> = string.bytes().collect();
    let mut bytes: Vec<u8> = Vec::with_capacity(string_as_numbers.len() / 2);

    for i in (0..string_as_numbers.len()).step_by(2) {
        bytes.push(convert_hex_to_byte(
            string_as_numbers[i],
            string_as_numbers[i + 1],
        ));
    }

    bytes
}

fn convert_byte_to_base64(byte: u8) -> char {
    if byte <= 25 {
        (65 + byte) as char
    } else if byte >= 26 && byte <= 51 {
        (97 + byte - 26) as char
    } else if byte >= 52 && byte <= 61 {
        (48 + byte - 52) as char
    } else if byte == 62 {
        '+'
    } else if byte == 63 {
        '/'
    } else {
        panic!()
    }
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

    #[test]
    fn test_convert_hex_to_byte() {
        let hex: &[u8] = "ff".as_bytes();
        assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 255u8);

        let hex: &[u8] = "00".as_bytes();
        assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 0u8);
    }

    #[test]
    fn test_convert_byte_to_base64() {
        assert_eq!(convert_byte_to_base64(0), 'A');
        assert_eq!(convert_byte_to_base64(26), 'a');
        assert_eq!(convert_byte_to_base64(52), '0');
        assert_eq!(convert_byte_to_base64(62), '+');
        assert_eq!(convert_byte_to_base64(63), '/');
    }

    #[test]
    fn test_convert_hex_sequence_to_bytes_sequence() {
        use hex::decode;
        let bytes = convert_hex_sequence_to_bytes_sequence("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
        let bytes_to_compare = decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()).unwrap();

        assert_eq!(bytes, bytes_to_compare);
    }

    #[test]
    fn test_convert_byte_to_bit_vector() {
        assert_eq!(convert_byte_to_bit_vector(0), vec![false; 8]);
        assert_eq!(convert_byte_to_bit_vector(255), vec![true; 8]);
        assert_eq!(
            convert_byte_to_bit_vector(5),
            vec![false, false, false, false, false, true, false, true]
        );
        assert_eq!(
            convert_byte_to_bit_vector(10),
            vec![false, false, false, false, true, false, true, false]
        );
    }

    #[test]
    fn test_convert_bit_vector_to_hex_byte_vector() {
        assert_eq!(
            convert_bit_vector_to_hex_byte_vector(vec![false, false, false, false, false, false]),
            vec![0]
        );

        assert_eq!(
            convert_bit_vector_to_hex_byte_vector(vec![false, true, true, true, true, true]),
            vec![31]
        );

        assert_eq!(
            convert_bit_vector_to_hex_byte_vector(vec![true, true, true, true, true, true]),
            vec![63]
        );
    }
}
