#![allow(dead_code)]
const BLOCK_SIZE: usize = 16;

pub fn pkcs7_padding(mut bytes: Vec<u8>) -> Vec<u8> {
    let extra_size = BLOCK_SIZE - (bytes.len() % BLOCK_SIZE);
    let mut extra_bytes = vec![extra_size as u8; extra_size];
    bytes.append(&mut extra_bytes);

    bytes
}

#[cfg(test)]
mod test {
    use super::pkcs7_padding;
    use crate::utils::encoding::hex;

    #[test]
    fn it_works() {
        let bytes = hex::decode("971ACD01C9C7ADEACC83257926F490FF".as_bytes());
        let target = hex::decode(
            "971ACD01C9C7ADEACC83257926F490FF10101010101010101010101010101010".as_bytes(),
        );

        assert_eq!(pkcs7_padding(bytes), target);

        let bytes = hex::decode("F14ADBDA019D6DB7EFD91546E3FF84449BCB".as_bytes());
        let target = hex::decode(
            "F14ADBDA019D6DB7EFD91546E3FF84449BCB0E0E0E0E0E0E0E0E0E0E0E0E0E0E".as_bytes(),
        );

        assert_eq!(pkcs7_padding(bytes), target);
    }
}
