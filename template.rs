use std::time::Instant;

fn part_one(input: &str) -> usize {
}

fn part_two(input: &str) -> usize {
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

    assert_eq!(295, part_one(&input));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");

    assert_eq!(1068781, part_two(&input));
}
