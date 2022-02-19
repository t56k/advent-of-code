use std::time::Instant;

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

static INPUT: &'static str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("part one: {:?}", part_one(INPUT));
    println!("part two: {:?}", part_two(INPUT));

    let end = start.elapsed();
    println!("time: {:}s", end.as_secs());
}

fn part_one(input: &str) -> u32 {
    find_suffix(input, "00000")
}

fn part_two(input: &str) -> u32 {
    find_suffix(input, "000000")
}

fn find_suffix(input: &str, prefix: &str) -> u32 {
    let mut hash = Md5::new();

    for i in 0..u32::MAX {
        hash.input_str(&format!("{}{:?}", input, i));

        if hash.result_str().starts_with(prefix) {
            return i;
        }

        hash.reset();
    }

    0
}
