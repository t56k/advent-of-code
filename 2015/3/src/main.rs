use std::collections::HashMap;
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
    let moves = input.lines().next().unwrap().chars();
    let mut grid = HashMap::new();
    let mut curr = (0, 0);
    grid.insert(curr, 1);

    for m in moves {
        curr = match m {
            '<' => (curr.0 - 1, curr.1),
            '>' => (curr.0 + 1, curr.1),
            '^' => (curr.0, curr.1 - 1),
            'v' => (curr.0, curr.1 + 1),
            _ => unreachable!(),
        };

        *grid.entry(curr).or_insert(0) += 1;
    }

    grid.iter().filter(|&(_, &v)| v > 0).count() as u32
}

fn part_two(input: &str) -> u32 {
    let moves = input.lines().next().unwrap().chars();
    let robo_moves = moves.clone().skip(1).step_by(2);

    let mut grid = HashMap::new();
    let mut santa_curr = (0, 0);
    let mut robo_santa_curr = (0, 0);

    grid.insert(santa_curr, 1);

    for m in moves.step_by(2) {
        santa_curr = match m {
            '<' => (santa_curr.0 - 1, santa_curr.1),
            '>' => (santa_curr.0 + 1, santa_curr.1),
            '^' => (santa_curr.0, santa_curr.1 - 1),
            'v' => (santa_curr.0, santa_curr.1 + 1),
            _ => unreachable!(),
        };

        *grid.entry(santa_curr).or_insert(0) += 1;
    }

    for m in robo_moves {
        robo_santa_curr = match m {
            '<' => (robo_santa_curr.0 - 1, robo_santa_curr.1),
            '>' => (robo_santa_curr.0 + 1, robo_santa_curr.1),
            '^' => (robo_santa_curr.0, robo_santa_curr.1 - 1),
            'v' => (robo_santa_curr.0, robo_santa_curr.1 + 1),
            _ => unreachable!(),
        };

        *grid.entry(robo_santa_curr).or_insert(0) += 1;
    }

    grid.iter().filter(|&(_, &v)| v > 0).count() as u32
}

#[test]
fn check_part_one() {
    let input = include_str!("../example_one");
    assert_eq!(5, part_one(&input));
}
