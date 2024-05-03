pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    if a.len() != b.len() {
        panic!();
    }

    a.into_iter()
        .zip(b.into_iter())
        .fold(0, |acc, (a, b)| (a ^ b).count_ones() + acc)
}

#[cfg(test)]
mod test {
    use super::hamming_distance;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
