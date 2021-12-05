use std::fs;
use std::mem::replace;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = input.lines().collect::<Vec<_>>();
    let binary_len: u32 = contents.iter().nth(0).unwrap().len() as u32;
    let cols: Vec<u32> = (0..binary_len).collect();

    let mut gamma_vec: Vec<u32> = cols.clone(); // most-common
    let mut epsilon_vec: Vec<u32> = cols.clone(); // least-common

    for i in cols {
        let col = nth_column(&contents, i);
        let zeroes = count(&col, 0);
        let most = most_common(zeroes, &col);

        let _ = replace(&mut gamma_vec[i as usize], most);
        let _ = replace(&mut epsilon_vec[i as usize], least_common(most));
    }

    let gamma_int = binary_vec_to_u32(&gamma_vec);
    let epsilon_int = binary_vec_to_u32(&epsilon_vec);
    let oxygen_results = filter_by_critera(&contents, &gamma_vec);
    let co2_results = filter_by_critera(&contents, &epsilon_vec);
    let oxygen_int = isize::from_str_radix(&oxygen_results.iter().nth(0).unwrap(), 2).unwrap() as u32;
    let co2_int = isize::from_str_radix(&co2_results.iter().nth(1).unwrap(), 2).unwrap() as u32;

    println!("part a: {}", gamma_int * epsilon_int);
    println!("part b: {}", oxygen_int * co2_int);
}

fn nth_column(contents: &Vec<&str>, n: u32) -> Vec<u32> {
    contents
        .iter()
        .map(|bin| bin.chars().nth(n as usize).unwrap().to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn count(column: &Vec<u32>, n: u32) -> u32 {
    column.iter().filter(|&i| *i == n).count() as u32
}

fn most_common(count: u32, column: &Vec<u32>) -> u32 {
    if count > column.len() as u32 / 2 {
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

fn binary_vec_to_u32(binary_vec: &Vec<u32>) -> u32 {
    let binary_str: String = binary_vec.iter().map(|&c| c.to_string()).collect();
    isize::from_str_radix(&binary_str, 2).unwrap() as u32
}

fn filter_by_critera(contents: &Vec<&str>, criteria: &Vec<u32>) -> Vec<String> {
    contents
        .iter()
        .filter(|bin| bin.chars().nth(0).unwrap().to_digit(10).unwrap() == *criteria.get(0).unwrap())
        .filter(|bin| bin.chars().nth(1).unwrap().to_digit(10).unwrap() == *criteria.get(1).unwrap())
        .filter(|bin| bin.chars().nth(2).unwrap().to_digit(10).unwrap() == *criteria.get(2).unwrap())
        .filter(|bin| bin.chars().nth(3).unwrap().to_digit(10).unwrap() == *criteria.get(3).unwrap())
        .filter(|bin| bin.chars().nth(4).unwrap().to_digit(10).unwrap() == *criteria.get(4).unwrap())
        .filter(|bin| bin.chars().nth(5).unwrap().to_digit(10).unwrap() == *criteria.get(5).unwrap())
        .filter(|bin| bin.chars().nth(6).unwrap().to_digit(10).unwrap() == *criteria.get(6).unwrap())
        .filter(|bin| bin.chars().nth(7).unwrap().to_digit(10).unwrap() == *criteria.get(7).unwrap())
        .filter(|bin| bin.chars().nth(8).unwrap().to_digit(10).unwrap() == *criteria.get(8).unwrap())
        .map(|bin| bin.to_string())
        .collect::<Vec<String>>()
}
