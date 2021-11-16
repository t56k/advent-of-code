#[macro_use]
extern crate partial_application;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let r = BufReader::new(f);

    for line in r.lines() {
        println!("{:?}", convert_to_row_col(&line.unwrap()));
    }

    Ok(())
}

fn calc_seat_id(row: u8, col: u8) -> u16 {
    row as u16 * 8 + col as u16
}

fn convert_to_row_col(line: &str) -> (u8, u8) {
    let row_str = &line[..7];
    let col_str = &line[7..];

    let convert_row_str_to_number = partial!(convert_code_to_number => 'F', 'B', _);
    let convert_col_str_to_number = partial!(convert_code_to_number => 'L', 'R', _);

    let row = convert_row_str_to_number(row_str);
    let col = convert_col_str_to_number(col_str);

    (row, col)
}

fn convert_code_to_number(zero_char: char, one_char: char, code: &str) -> u8 {
    code.chars()
        .map(|c| {
            if c == zero_char {
                0
            } else if c == one_char {
                1
            } else {
                panic!("unknown {}", c)
            }
        })
        .fold(0, |acc, cur| acc * 2 + cur)
}
