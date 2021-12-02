use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = input.lines().collect::<Vec<_>>();

    let horizontal =
        contents
            .iter()
            .filter(|el| el.contains("forward"))
            .fold(0, |acc, el| acc
                + el.split(" ").nth(1).unwrap().parse::<i32>().unwrap());

    let up =
        contents
            .iter()
            .filter(|el| el.contains("up"))
            .fold(0, |acc, el| acc
                + el.split(" ").nth(1).unwrap().parse::<i32>().unwrap());

    let down =
        contents
            .iter()
            .filter(|el| el.contains("down"))
            .fold(0, |acc, el| acc
                + el.split(" ").nth(1).unwrap().parse::<i32>().unwrap());

    let depth = down - up;

    println!("{}", horizontal * depth);
}
