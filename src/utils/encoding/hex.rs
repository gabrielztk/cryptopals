pub fn decode(hex: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        bytes.push(convert_hex_to_byte(hex[i], hex[i + 1]));
    }

    bytes
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        use hex;

        let bytes = encode(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".bytes().collect::<Vec<u8>>());
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
    fn test_decode_hex() {
        use hex;

        let bytes = decode(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".bytes().collect::<Vec<u8>>());
        let bytes_to_compare = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();

        assert_eq!(bytes, bytes_to_compare);
    }

    #[test]
    fn test_convert_hex_to_byte() {
        let hex = "ff".as_bytes();
        assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 255u8);

        let hex = "00".as_bytes();
        assert_eq!(convert_hex_to_byte(hex[0], hex[1]), 0u8);
    }
}
