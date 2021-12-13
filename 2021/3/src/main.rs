use std::fs;
use std::mem::replace;

struct Candidates<'a> {
    o: Vec<&'a str>,
    co2: Vec<&'a str>,
}

struct GasRates {
    o: Option<u32>,
    co2: Option<u32>,
}

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
    let len = contents[0].len() as u32;
    let cols: Vec<u32> = (0 as u32..len).collect();

    let mut gamma_vec: Vec<u32> = cols.clone();
    let mut epsilon_vec: Vec<u32> = cols.clone();

    for i in cols {
        let col = nth_column(&contents, i as usize);
        let zeroes = count(&col, 0);
        let most = most_common(zeroes, &col);

        let _ = replace(&mut gamma_vec[i as usize], most);
        let _ = replace(&mut epsilon_vec[i as usize], least_common(most));
    }

    let gamma_int = binary_vec_to_usize(&gamma_vec);
    let epsilon_int = binary_vec_to_usize(&epsilon_vec);

    (gamma_int * epsilon_int).try_into().unwrap()
}

fn part_b(contents: &Vec<&str>) -> u32 {
    let len = contents[0].len();

    let (_, nums) = (0..len).fold(
        (
            Candidates {
                o: contents.clone(),
                co2: contents.clone(),
            },
            GasRates { o: None, co2: None },
        ),
        |(mut candidates, mut final_nums), i| {
            let (o_zeroes, o_ones) = count_ones_zeroes(&candidates.o, i);
            let most_common = if o_ones >= o_zeroes { '1' } else { '0' };

            let (co2_zeroes, co2_ones) = count_ones_zeroes(&candidates.co2, i);
            let least_common = if co2_zeroes <= co2_ones { '0' } else { '1' };

            candidates.o = filter_by_bit(candidates.o, i, most_common);
            candidates.co2 = filter_by_bit(candidates.co2, i, least_common);

            if let None = final_nums.co2 {
                if candidates.co2.len() == 1 {
                    final_nums.co2 = Some(u32::from_str_radix(candidates.co2[0], 2).unwrap());
                }
            }
            if let None = final_nums.o {
                if candidates.o.len() == 1 {
                    final_nums. o= Some(u32::from_str_radix(candidates.o[0], 2).unwrap());
                }
            }

            (candidates, final_nums)
        }
    );

    nums.o.unwrap() * nums.co2.unwrap()
}

fn nth_column(contents: &Vec<&str>, n: usize) -> Vec<u32> {
    contents
        .iter()
        .map(|bin| bin.chars().nth(n).unwrap().to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn count(column: &Vec<u32>, n: u32) -> u32 {
    column.iter().filter(|&i| *i == n).count() as u32
}

fn most_common(count: u32, column: &Vec<u32>) -> u32 {
    if count > (column.len() / 2).try_into().unwrap() { 0 } else { 1 }
}

fn least_common(most_common: u32) -> u32 {
    if most_common == 0 { 1 } else { 0 }
}

fn binary_vec_to_usize(binary_vec: &Vec<u32>) -> u32 {
    let binary_str: String = binary_vec.iter().map(|&c| c.to_string()).collect();
    isize::from_str_radix(&binary_str, 2).unwrap() as u32
}

fn count_ones_zeroes(data: &Vec<&str>, i: usize) -> (u32, u32) {
    data.iter().fold((0, 0), |(zeroes, ones), bin| {
        let bit = bin.chars().nth(i).unwrap();
        if bit == '1' {
            (zeroes, ones + 1)
        } else {
            (zeroes + 1, ones)
        }
    })
}

fn filter_by_bit(contents: Vec<&str>, i: usize, bit: char) -> Vec<&str> {
    contents
        .iter()
        .filter(|bin| bin.chars().nth(i).unwrap() == bit)
        .map(|bin| bin.to_owned())
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = parse(input);
        assert_eq!(part_a(&data), 198);
    }

    #[test]
    fn part_two_example() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = parse(input);
        assert_eq!(part_b(&data), 230);
    }
}
