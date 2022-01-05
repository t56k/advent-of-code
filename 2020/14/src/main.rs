use regex::Regex;
use std::time::Instant;
use std::collections::HashMap;

fn part_one(input: &str) -> usize {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut and_or = (0, 0);

    for line in input.lines() {
        if line.starts_with("ma") {
            and_or = line
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .fold((usize::MAX, 0), |(and, or), (i, b)| match b {
                    b'0' => (and & !(1 << i), or),
                    b'1' => (and, or | 1 << i),
                    _ => (and, or),
                });
        } else {
            let captures = re.captures(&line).unwrap();
            mem.insert(
                captures[1].parse().unwrap(),
                captures[2].parse::<usize>().unwrap() & and_or.0 | and_or.1,
            );
        }
    }

    mem.values().sum::<usize>()
}

fn part_two(input: &str) -> usize {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let mut mem = HashMap::new();
    let mut float_addrs = vec![];
    let mut whitelist = 0;

    for line in input.lines() {
        if line.starts_with("me") {
            let cap = re.captures(&line).unwrap();
            let addr = cap[1].parse::<usize>().unwrap() & whitelist;
            let val = cap[2].parse().unwrap();
            for float_addr in &float_addrs {
                mem.insert(addr | float_addr, val);
            }
        } else {
            let mut float_base = 0;
            let mut float_bits = vec![];
            float_addrs.clear();
            whitelist = 0;

            line.split(" = ")
                .nth(1)
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .for_each(|(i, b)| match b {
                    b'0' => whitelist |= 1 << i,
                    b'1' => float_base |= 1 << i,
                    b'X' => float_bits.push(i),
                    _ => unreachable!(),
                });

            float_addrs = (0..2usize.pow(float_bits.len() as u32))
                .map(|template| float_bits
                    .iter()
                    .enumerate()
                    .fold(float_base, |addr, (i, fb)| addr | (template & 1 << i) << fb - i)
                )
                .collect();
        }
    }

    mem.values().sum::<usize>()
}

fn main() {
    let input = include_str!("../input");

    benchmark(|| println!("part one: {}", part_one(&input)));
    benchmark(|| println!("part two: {}", part_two(&input)));
}

fn benchmark<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = f();
    println!("time: {}µs", start.elapsed().as_micros());

    result
}

#[test]
fn check_part_one() {
    let input = include_str!("../example_one");

    assert_eq!(165, part_one(&input));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");

    assert_eq!(208, part_two(&input));
}

