use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use once_cell::sync::Lazy;

static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<num>\d+) (?<color>red|green|blue),?").unwrap());
static MAX_AMOUNT: Lazy<HashMap<&str, u8>> = Lazy::new(|| HashMap::from([
    ("red", 12),
    ("green", 13),
    ("blue", 14)
]));

fn main() {
    let mut part_1: u32 = 0;
    let mut part_2: u32 = 0;

    let file = File::open("./input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        part_1 += get_valid_cubes(&l).unwrap_or_default();
        part_2 += get_min_power(&l);
    }
    println!("Result of Part 1: {} Part 2: {}", part_1, part_2);
}

fn get_valid_cubes(line: &String) -> Option<u32> {
    let mut chunks = line.split(&[':',';']);
    let id = chunks.next().unwrap()[5..].parse::<u32>().unwrap();
    for pull in chunks {
        for cubes in PATTERN.captures_iter(pull) {
            let amount = cubes.name("num").unwrap().as_str().parse::<u8>().unwrap();
            let color = cubes.name("color").unwrap().as_str();
            if &amount > MAX_AMOUNT.get(color).unwrap() {
                return None;
            }
        }
    }

    Some(id)
}

fn get_min_power(line: &String) -> u32 {
    let mut chunks = line.split(&[':',';']);
    chunks.next();
    let mut min: HashMap<&str, u32> = HashMap::with_capacity(3);
    for pull in chunks {
        for cubes in PATTERN.captures_iter(pull) {
            let amount = cubes.name("num").unwrap().as_str().parse::<u32>().unwrap();
            let color = cubes.name("color").unwrap().as_str();
            if &amount > min.get(color).unwrap_or(&0) {
                min.insert(color, amount);
            }
        }
    }
    min.into_values().reduce(|acc, v| acc * v).unwrap_or(0)
}