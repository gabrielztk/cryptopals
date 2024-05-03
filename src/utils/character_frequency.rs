const SIZE: usize = 28;
const FREQUENCY: [f64; SIZE] = [
    0.07640468802677386,
    0.013902683240920158,
    0.026572857564631135,
    0.03099641943289172,
    0.0984284900007756,
    0.017662050446609373,
    0.01566380517852733,
    0.039621054850953005,
    0.06288815831069514,
    0.002235133715648028,
    0.006564266631899178,
    0.03450370307360186,
    0.023195054633102116,
    0.06169992783312964,
    0.05932003999658444,
    0.016774704975322297,
    0.0006527938075325359,
    0.054184048152489936,
    0.05614501930972394,
    0.06876466369483174,
    0.021642472582750424,
    0.008358717852012903,
    0.01415529679810389,
    0.0014313824693683477,
    0.01330353307619316,
    0.0013350684314491642,
    0.1735939659134791,
    0.0,
];

pub fn score_byte_sequence(bytes: &Vec<u8>) -> f64 {
    find_frequency(bytes)
        .iter()
        .zip(FREQUENCY.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0_f64, |acc, n| acc + n)
}

fn find_frequency(bytes: &Vec<u8>) -> [f64; SIZE] {
    let (char_count, mut letter_count) = find_count(bytes);

    for i in 0..letter_count.len() {
        letter_count[i] /= char_count as f64;
    }

    letter_count
}

fn find_count(bytes: &Vec<u8>) -> (u64, [f64; SIZE]) {
    let mut letter_count = [0_f64; SIZE];
    let mut count = 0_u64;

    bytes.iter().for_each(|c| {
        let c = c.to_ascii_lowercase() as u8;
        if c >= 97 && c <= 122 {
            letter_count[(c - 97) as usize] += 1_f64;
        } else if c == 32 {
            letter_count[26] += 1_f64;
        } else {
            letter_count[27] += 1_f64;
        }
        count += 1;
    });

    (count, letter_count)
}

pub fn find_wikipedia_letter_frequency() {
    use std::fs::File;
    use std::io::prelude::*;

    // Wikipedia words file found at the following link
    // https://www.kaggle.com/datasets/ffatty/plain-text-wikipedia-simpleenglish?resource=download

    let mut file = File::open("files/set_1/AllCombined.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut letter_count = [0_u64; SIZE];
    let mut count = 0_u64;

    contents.chars().for_each(|c| {
        let c = c.to_ascii_lowercase() as u8;
        if c >= 97 && c <= 122 {
            letter_count[(c - 97) as usize] += 1;
            count += 1;
        } else if c == 32 {
            letter_count[26] += 1;
            count += 1;
        }
    });

    let mut frequency = [0_f64; SIZE];
    for i in 0..SIZE {
        frequency[i] = (letter_count[i] as f64) / count as f64;
    }

    let mut file = File::create("files/set_1/character_frequency.txt").unwrap();
    let frequency = format!("{frequency:?}");
    file.write_all(frequency.as_bytes()).unwrap();
}
