use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn part_one(nums: &Vec<usize>) -> usize {
    nums.windows(26)
        .find(|set| {
            for i in 0..24 {
                for j in (i + 1)..25 {
                    if set[i] + set[j] == set[25] {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()[25]
}

fn part_two(nums: &Vec<usize>, target: usize) -> usize {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let span = nums[i..j].to_vec();
            if span.iter().sum::<usize>() == target {
                let min_v: usize = *span.iter().min().unwrap();
                let max_v: usize = *span.iter().max().unwrap();
                return min_v + max_v;
            }
        }
    }
    1
}

fn main() {
    let input = include_str!("../input");
    let nums = parse(&input);
    let target = part_one(&nums); // do it twice so we can benchmark it

    benchmark(|| println!("part one: {}", part_one(&nums)));
    benchmark(|| println!("part two: {}", part_two(&nums, target)));
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
    let nums = parse(&input);

    assert_eq!(127, part_one(&nums));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");
    let nums = parse(&input);

    assert_eq!(62, part_two(&nums, 127));
}
