#![allow(dead_code)]
use crate::utils::encoding::hex;

fn fixed_xor(string_a: String, string_b: String) -> String {
    let bytes_a = hex::decode(&string_a.bytes().collect::<Vec<u8>>());
    let bytes_b = hex::decode(&string_b.bytes().collect::<Vec<u8>>());

    let bytes_xor = xor_bytes(bytes_a, bytes_b);
    let bytes_hex = hex::encode(&bytes_xor);

    String::from_utf8_lossy(&bytes_hex).to_string()
}

fn xor_bytes(bytes_a: Vec<u8>, bytes_b: Vec<u8>) -> Vec<u8> {
    bytes_a
        .iter()
        .zip(bytes_b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::set_1_basics::c_02_fixed_xor::fixed_xor;

    #[test]
    fn it_works() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c".to_string(),
                "686974207468652062756c6c277320657965".to_string()
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }
}
