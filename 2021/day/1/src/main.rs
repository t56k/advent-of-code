 use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse_input(input);
    let n = contents.len();
    let mut counter = 0;

    for i in 0..n {
        match contents.get(i + 1) {
            Some(x) => {
                if x > &contents[i] {
                    counter += 1;
                }
            },
            None => ()
        }
    }

    println!("part a: {:?}", counter);
}

fn parse_input(input: String) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
