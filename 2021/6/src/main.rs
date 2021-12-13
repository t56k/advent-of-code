use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let part_one = count_fish(&input, 80);
    let part_two = count_fish(&input, 256);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

// arrays need to be indexed by a usize type
fn count_fish(input: &str, days: usize) -> usize {
    let mut lanternfish = [0usize; 9];

    for f in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        lanternfish[f] += 1;
    }

    for d in 0..days {
        lanternfish[(d + 7) % 9] += lanternfish[d % 9];
    }
    lanternfish.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "3,4,3,1,2";

        assert_eq!(count_fish(&input, 80), 5934);
    }

    #[test]
    fn part_twoe_example() {
        let input = "3,4,3,1,2";

        assert_eq!(count_fish(&input, 256), 26984457539);
    }
}
