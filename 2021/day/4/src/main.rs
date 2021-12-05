use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse(&input);
    let part_a = part_a(&contents);
    let part_b = part_b(&contents);

    println!("{}", part_a.clone());
    println!("{}", part_b);
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_a(contents: &Vec<&str>) -> u32 {
}

fn part_b(contents: &Vec<&str>) -> u32 {
}
