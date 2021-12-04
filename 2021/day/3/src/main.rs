use std::fs;
use std::mem::replace;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = input.lines().collect::<Vec<_>>();
    let binary_len: u32 = contents.iter().nth(0).unwrap().len() as u32;
    let cols: Vec<u32> = (0..binary_len).collect();

    let mut gamma: Vec<u32> = cols.clone();
    let mut epsilon: Vec<u32> = cols.clone();

    for i in cols {
        let c = nth_column(&contents, i);
        let z = zero_count(&c);
        let m = most_common(z, &c);

        let _ = replace(&mut gamma[i as usize], m);
        let _ = replace(&mut epsilon[i as usize], least_common(m));
    }

    let gamma_str: String = gamma.iter().map( |&c| c.to_string()).collect();
    let gamma_int = isize::from_str_radix(&gamma_str, 2).unwrap();

    let epsilon_str: String = epsilon.iter().map( |&c| c.to_string()).collect();
    let epsilon_int = isize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("{}", gamma_int * epsilon_int);
}

fn nth_column(contents: &Vec<&str>, n: u32) -> Vec<u32> {
    contents
        .iter()
        .map(|c| c.chars().nth(n as usize).unwrap().to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn zero_count(column: &Vec<u32>) -> u32 {
    column
        .iter()
        .filter(|&i| *i == 0)
        .count() as u32
}

fn most_common(zero_count: u32, column: &Vec<u32>) -> u32 {
    if zero_count > column.len() as u32 / 2 {
        0
    } else {
        1
    }
}

fn least_common(most_common: u32) -> u32 {
    if most_common == 0 {
        1
    } else {
        0
    }
}