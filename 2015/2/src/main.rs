use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));

    let end = start.elapsed();
    println!("time: {:}us", end.as_micros());
}

fn part_one(input: &str) -> u32 {
    input.lines().into_iter().fold(0, |acc, line| {
        let sides = parse_line(line);
        let area = sides
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut j = i + 1;
                if j == sides.len() {
                    j = 0
                }

                sides[i] * sides[j]
            })
            .collect::<Vec<u32>>();

        let min = area.iter().min().unwrap();
        acc + (2 * area.iter().sum::<u32>()) + min
    })
}

fn part_two(input: &str) -> u32 {
    input.lines().into_iter().fold(0, |acc, line| {
        let sides = parse_line(line);
        let mut smallest = sides.clone();

        smallest.sort();
        smallest.pop();

        acc + ((2 * smallest.iter().sum::<u32>()) + sides.iter().product::<u32>())
    })
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split('x')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[test]
fn check_part_one() {
    let input = include_str!("../example_one");
    assert_eq!(101, part_one(&input));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");
    assert_eq!(48, part_two(&input));
}
