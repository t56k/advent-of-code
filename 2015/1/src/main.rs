use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));

    let end = start.elapsed();
    println!("time: {:}us", end.as_micros());
}

pub fn part_one(input: &str) -> usize {
    let up = input.matches('(').collect::<Vec<&str>>().len();
    let down = input.matches(')').collect::<Vec<&str>>().len();

    up - down
}

pub fn part_two(input: &str) -> usize {
    let mut res = 0i16;

    for (i, el) in input.chars().into_iter().enumerate() {
        if el == '(' {
            res += 1;
        } else if el == ')' {
            res -= 1;
        }

        if res < 0 { return i + 1 }
    }

    0
}

// #[test]
// fn check_part_one() {
//     let input = include_str!("../example_one");
//     assert_eq!(35, part_one(input));
// }

