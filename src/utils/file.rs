pub fn read_bytes(path: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let bytes =
        reader
            .lines()
            .map(|l| l.unwrap().bytes().collect())
            .fold(vec![], |mut acc, mut line| {
                acc.append(&mut line);
                acc
            });
    bytes
}

pub fn read_lines(path: &str) -> Vec<Vec<u8>> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();

    lines
}
