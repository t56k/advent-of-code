use std::fs;

// learned about https://doc.rust-lang.org/std/iter/struct.Zip.html
pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse_input(input);

    println!("part a: {}", count_windows(&contents, 1));
    println!("part b: {}", count_windows(&contents, 3));
}

fn count_windows(contents: &Vec<usize>, span: usize) -> usize {
    contents
        .iter()
        .zip(contents[span..].iter())
        .filter(|( a, b )| a < b)
        .count()
}

fn parse_input(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}
