use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse_input(input);

    println!("part a: {}",
             contents
                .iter()
                .zip(contents[1..].iter())
                .filter(|( a, b )| a < b)
                .count()
    );

}

fn parse_input(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}
