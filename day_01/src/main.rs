use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;

static NUMERIC_WORDS: Lazy<Vec<&str>> = Lazy::new(|| vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]);

fn main() {
    let mut coords: Vec<u8> = Vec::new();

    let file = File::open("./input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        match get_coord(line.unwrap()) {
            None => (),
            Some(coord) => coords.push(coord)
        }
    }

    let sum: u32 = coords.iter().map(|&x| x as u32).sum();
    println!("Summary of calibration values: {}", sum);
}

fn get_coord(line: String) -> Option<u8> {
    println!("{}", line);
    let digits = parse_digits(&line);
    println!("{:?}", digits);
    if digits.len() == 0 {
        return None;
    }
    
    Some(digits.first().unwrap() * 10 + digits.last().unwrap())
}

fn parse_digits(line: &str) -> Vec<u8> {
    let mut positions: Vec<(usize, u8)> = line
        .match_indices(char::is_numeric)
        .map(|(i, chr)| (i, chr.parse::<u8>().unwrap()))
        .collect();

    for (i, &word) in NUMERIC_WORDS.iter().enumerate() {
        positions.extend(line
            .match_indices(word)
            .map(|(pos, _)| (pos, (i + 1) as u8))
        );
    }
    positions.sort_by(|a, b| a.0.cmp(&b.0));

    positions.into_iter().map(|(_, digit)| digit).collect()
}
