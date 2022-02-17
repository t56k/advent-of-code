use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input");

    println!("part one: {}", part_one(&input));

    let end = start.elapsed();
    println!("time: {:}us", end.as_micros());
}

pub fn part_one(input: &str) -> u32 {
    input.lines().into_iter().fold(0, |acc, line| {
        let sides = line
            .split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let area = sides
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut j = i + 1;
                if j == 3 { j = 0 }

                sides[i] * sides[j]
            })
            .collect::<Vec<u32>>();

        let min = area.iter().min().unwrap();
        acc + (2 * area.iter().sum::<u32>()) + min
    })
}

#[test]
fn check_part_one() {
    let input = include_str!("../example_one");
    assert_eq!(101, part_one(&input));
}
