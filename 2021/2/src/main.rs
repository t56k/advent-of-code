use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = input.lines().collect::<Vec<_>>();

    let horizontal = calculate_by_type(&contents, "forward");
    let up = calculate_by_type(&contents, "up");
    let down = calculate_by_type(&contents, "down");
    let depth = down - up;

    println!("part a: {}", horizontal * depth);
    println!("part b: {}", calculate_aim_and_depth(&contents));
}

// part a
fn calculate_by_type(contents: &Vec<&str>, direction: &str) -> i32 {
    contents
        .iter()
        .filter(|el| el.contains(direction))
        .fold(0, |acc, el| {
            acc + el.split(" ").nth(1).unwrap().parse::<i32>().unwrap()
        })
}

// part b
fn calculate_aim_and_depth(contents: &Vec<&str>) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    contents.iter().for_each(|el| {
        let parts = el.split(" ").collect::<Vec<_>>();
        let dir = parts[0];
        let val = parts[1].parse::<i32>().unwrap();

        match dir {
            "down" => aim += val,
            "up" => aim -= val,
            "forward" => {
                horizontal += val;
                depth += aim * val;
            }
            _ => (),
        }
    });

    horizontal * depth
}
