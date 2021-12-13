use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let part_one = linear_fuel_cost(&input);
    let part_two = exp_fuel_cost(&input);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn linear_fuel_cost(input: &str) -> i64 {
    let crabs: Vec<_> = input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    (0..=*crabs.iter().max().unwrap())
        .map(|target| crabs.iter().map(|d| (d - target).abs()).sum())
        .min()
        .unwrap()
}

fn exp_fuel_cost(input: &str) -> i64 {
    let crabs: Vec<_> = input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    (0..=*crabs.iter().max().unwrap())
        .map(|target| {
            crabs
                .iter()
                .map(|d| (((d - target).pow(2) + (d - target).abs()) / 2))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(linear_fuel_cost(&input), 37);
    }

    #[test]
    fn part_two_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(exp_fuel_cost(&input), 168);
    }
}
