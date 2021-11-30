use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse_input(input);
    // let groups = contents.chunks(25);

    let res = check_for_factors(&contents, 222);
    println!("{}", res);
}

// fn check_for_factors(contents: Vec<isize>, target: isize) -> bool {
//     contents.iter().enumerate().any(|(i, &x)| {
//         contents
//             .iter()
//             .enumerate()
//             .filter(|&(j, _)| i != j)
//             .any(|(_, &y)| x + y == target)
//     })
// }

fn parse_input(input: String) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn check_for_factors(contents: Vec<isize>, target: isize) -> bool {
    let n = contents.len();
    for i in 0..n {
        let x = contents[i];
        for j in (i + 1)..n {
            let y = contents[j];
            if x + y == target {
                return true;
            }
        }
    }
    false
}
