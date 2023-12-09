use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;

static NUMERIC_WORDS: Lazy<Vec<&str>> = Lazy::new(|| vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]);

enum NumericMatch<'a> {
    None,
    Partial(Vec<&'a str>),
    Full((&'a str, u32))
}

fn main() {
    let mut coords: Vec<u32> = Vec::new();

    let file = File::open("./input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        match get_coord(line.unwrap()) {
            None => (),
            Some(coord) => coords.push(coord)
        }
    }

    let sum: u32 = coords.iter().sum();
    println!("Summary of calibration values: {}", sum);
}

fn get_coord(line: String) -> Option<u32> {
    println!("{}", line);
    let digits = parse_digits(&line);
    println!("{:?}", digits);
    if digits.len() == 0 {
        return None;
    }
    
    Some(digits.first().unwrap() * 10 + digits.last().unwrap())
}

fn parse_digits(line: &str) -> Vec<u32> {
    let mut parsed: Vec<u32> = Vec::new();
    let mut buffer = (0, 1);
    let mut i: usize = 0;
    while buffer.1 <= line.len() {
        let slc: &str = &line[buffer.0..buffer.1];
        let chr = slc.chars().next().unwrap();
        if chr.is_numeric() {
            i += 1;
            buffer = (i, i + 1);
            parsed.push(chr.to_digit(10).unwrap());
            continue;
        }
        match check_for_digit(slc) {
            NumericMatch::None => {
                if buffer.1 - buffer.0 > 1 {
                    buffer = (i, i + 1);
                } else {
                    i += 1;
                    buffer = (i, i + 1);
                }
            },
            NumericMatch::Partial(_) => {
                buffer.1 += 1;
                i += 1;
            },
            NumericMatch::Full((_, num)) => {
                parsed.push(num);
                buffer = (i, i + 1);
            }
        }
    }
    parsed
}

fn check_for_digit(text: &str) -> NumericMatch {
    println!("{}", text);
    let mut partials: Vec<&str> = Vec::new();
    for (i, digit) in NUMERIC_WORDS.clone().into_iter().enumerate() {
        if digit.starts_with(text) {
            partials.push(digit);
            if digit.len() == text.len() {
                return NumericMatch::Full((digit, u32::try_from(i).unwrap() + 1));
            }
        }
    }
    if partials.len() > 0 {
        return NumericMatch::Partial(partials);
    }
    NumericMatch::None
}