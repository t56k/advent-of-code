use std::time::Instant;

fn part_one(input: &str) -> usize {
    let (est, lines) = input.split_once('\n').unwrap();
    let est = est.parse::<usize>().unwrap();
    let (line, wait) = lines
        .split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .map(|l| (l, l - (est % l)))
        .min_by_key(|l| l.1)
        .unwrap();

    line * wait
}

fn part_two(input: &str) -> i64 {
    let lines: Vec<(i64, i64)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|l| (l - i as i64, l)))
        .collect();

    crt(&lines)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn crt(remo: &[(i64, i64)]) -> i64 {
    let prod = remo.iter().map(|n| n.1).product::<i64>();
    remo.iter()
        .map(|(re, mo)| {
            let p = prod / mo;
            re * ((egcd(p, *mo).1 % mo + mo) % mo) * p
        })
        .sum::<i64>()
        % prod
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
