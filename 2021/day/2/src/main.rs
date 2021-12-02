use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = input.lines().collect::<Vec<_>>();

    let horizontal = calculate_by_type(&contents, "forward");
    let up = calculate_by_type(&contents, "up");
    let down = calculate_by_type(&contents, "down");
    let depth = down - up;

    println!("{}", horizontal * depth);
}

fn calculate_by_type(contents: &Vec<&str>, direction: &str) -> i32 {
    contents
        .iter()
        .filter(|el| el.contains(direction))
        .fold(0, |acc, el| acc
            + el.split(" ").nth(1).unwrap().parse::<i32>().unwrap())
}
