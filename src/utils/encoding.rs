pub fn decode_hex(hex: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        bytes.push(convert_hex_to_byte(hex[i], hex[i + 1]));
    }

    bytes
}

pub fn encode_hex(bytes: &[u8]) -> Vec<u8> {
    let mut hex: Vec<u8> = Vec::with_capacity(bytes.len() * 2);

    for i in 0..bytes.len() {
        let (most, least) = convert_byte_to_hex(bytes[i]);
        hex.push(most);
        hex.push(least);
    }

    hex
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
            panic!("({})", byte)
        }
    }

    map_to_byte(least) + (map_to_byte(most) << 4)
}

fn convert_byte_to_hex(byte: u8) -> (u8, u8) {
    fn map_to_hex(byte: u8) -> u8 {
        if byte <= 9 {
            byte + 48
        } else if byte <= 15 {
            byte + 97 - 10
        } else {
            panic!("({})", byte)
        }
    }

    let (most, least) = ((byte & 0b1111_0000) >> 4, byte & 0b1111);

    (map_to_hex(most), map_to_hex(least))
}

pub fn encode_64(bytes: &[u8]) -> Vec<u8> {
    let bit_vec: Vec<bool> = convert_byte_vector_to_bit_vector(bytes);
    let hex_vec: Vec<u8> = convert_bit_vector_to_hex_byte_vector(bit_vec);

    hex_vec
        .into_iter()
        .map(|byte| convert_byte_to_base64(byte))
        .collect()
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

fn convert_byte_vector_to_bit_vector(bytes: &[u8]) -> Vec<bool> {
    let size = bytes.len();
    let bit_vec: Vec<bool> = bytes
        .into_iter()
        .fold(Vec::with_capacity(size * 8), |mut acc, &b| {
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
        panic!()
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
        panic!()
    }
}

#[cfg(test)]
mod test {
    use crate::utils::encoding::*;

    #[test]
    fn test_decode_hex() {
        use hex;

        let bytes = decode_hex(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".bytes().collect::<Vec<u8>>());
        let bytes_to_compare = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();

        assert_eq!(bytes, bytes_to_compare);
    }

    #[test]
    fn test_encode_hex() {
        use hex;

        let bytes = encode_hex(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".bytes().collect::<Vec<u8>>());
        let string_to_compare = hex::encode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

        assert_eq!(
            bytes
                .into_iter()
                .map(|byte| byte as char)
                .collect::<String>(),
            string_to_compare
        );
    }

    #[test]
    fn test_encode_64() {
        use base64::prelude::*;
        use hex;

        let bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let bytes_to_compare = BASE64_STANDARD.encode(&bytes);
        let bytes = encode_64(&bytes);

        assert_eq!(
            bytes
                .into_iter()
                .map(|byte| byte as char)
                .collect::<String>(),
            bytes_to_compare
        );
    }

    // #[test]
    // fn test_convert_hex_to_byte() {
    //     let hex: &[u8] = "ff".as_bytes();
    //     assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 255u8);

    //     let hex: &[u8] = "00".as_bytes();
    //     assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 0u8);
    // }

    // #[test]
    // fn test_convert_byte_to_base64() {
    //     assert_eq!(convert_byte_to_base64(0), 'A');
    //     assert_eq!(convert_byte_to_base64(26), 'a');
    //     assert_eq!(convert_byte_to_base64(52), '0');
    //     assert_eq!(convert_byte_to_base64(62), '+');
    //     assert_eq!(convert_byte_to_base64(63), '/');
    // }

    // #[test]
    // fn test_convert_hex_sequence_to_bytes_sequence() {
    //     use hex::decode;
    //     let bytes = convert_hex_sequence_to_bytes_sequence("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
    //     let bytes_to_compare = decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()).unwrap();

    //     assert_eq!(bytes, bytes_to_compare);
    // }

    // #[test]
    // fn test_convert_byte_to_bit_vector() {
    //     assert_eq!(convert_byte_to_bit_vector(0), vec![false; 8]);
    //     assert_eq!(convert_byte_to_bit_vector(255), vec![true; 8]);
    //     assert_eq!(
    //         convert_byte_to_bit_vector(5),
    //         vec![false, false, false, false, false, true, false, true]
    //     );
    //     assert_eq!(
    //         convert_byte_to_bit_vector(10),
    //         vec![false, false, false, false, true, false, true, false]
    //     );
    // }

    // #[test]
    // fn test_convert_bit_vector_to_hex_byte_vector() {
    //     assert_eq!(
    //         convert_bit_vector_to_hex_byte_vector(vec![false, false, false, false, false, false]),
    //         vec![0]
    //     );

    //     assert_eq!(
    //         convert_bit_vector_to_hex_byte_vector(vec![false, true, true, true, true, true]),
    //         vec![31]
    //     );

    //     assert_eq!(
    //         convert_bit_vector_to_hex_byte_vector(vec![true, true, true, true, true, true]),
    //         vec![63]
    //     );
    // }
}
