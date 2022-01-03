use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn part_one(adapters: &Vec<usize>) -> usize {
    let (one, three) =
        adapters
            .windows(2)
            .fold((1, 1), |(one, three), val| match val[1] - val[0] {
                1 => (one + 1, three),
                3 => (one, three + 1),
                _ => unreachable!(),
            });

    one * three
}

fn part_two(adapters: &Vec<usize>) -> usize {
    let mut adapters_clone = adapters.clone();
    adapters_clone.insert(0, 0);

    let mut dp_adapters = vec![0 as i128; adapters_clone.len()];
    dp_adapters[0] = 1;

    for i in 1..dp_adapters.len() {
        let mut sum = 0;
        let mut idx = (i as isize) - 3;
        if idx < 0 {
            idx = 0;
        }

        for j in idx as usize..i {
            if (adapters_clone[i] - adapters_clone[j]) <= 3 {
                sum += dp_adapters[j];
            }
        }
        dp_adapters[i] = sum;
    }
    dp_adapters[dp_adapters.len() - 1] as usize
}

fn main() {
    let input = include_str!("../input");
    let mut adapters = parse(&input);
    adapters.sort();

    benchmark(|| println!("part one: {}", part_one(&adapters)));
    benchmark(|| println!("part two: {}", part_two(&adapters)));
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
    let mut adapters = parse(&input);
    adapters.sort();

    assert_eq!(220, part_one(&adapters));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");
    let mut adapters = parse(&input);
    adapters.sort();

    assert_eq!(19208, part_two(&adapters));
}
