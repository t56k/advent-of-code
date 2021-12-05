use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Data {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone)]
struct Board {
    i: usize,
    grid: Vec<Vec<(u8, bool)>>,
}

impl Board {
    fn mark(&mut self, called_n: u8) {
        if let Some(square) = self.grid.iter_mut().flatten().find(|(n, _)| *n == called_n) {
            square.1 = true;
        }
    }

    fn check_win(&self) -> bool {
        let mut row_candidates: HashSet<usize> = (0..5).collect();
        let mut col_candidates: HashSet<usize> = (0..5).collect();

        for row_i in 0..5 {
            for col_i in 0..5 {
                let result = self.grid[row_i][col_i].1;
                if result == false {
                    row_candidates.remove(&row_i);
                    col_candidates.remove(&col_i);
                }
            }
        }

        !row_candidates.is_empty() || !col_candidates.is_empty()
    }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter_map(|(num, mark)| if *mark { None } else { Some(*num as u32) })
            .sum()
    }
}

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let contents = parse(&input);
    let part_a = part_a(contents.clone());
    let part_b = part_b(contents);

    println!("{}", part_a);
    println!("{}", part_b);
}

fn parse(input: &str) -> Data {
    let (numbers, rest) = input.split_once("\n\n").unwrap();
    let numbers = numbers.split(",").filter_map(|s| s.parse().ok()).collect();

    let boards = rest
        .split("\n\n")
        .enumerate()
        .map(|(i, block)| parse_block(i, block))
        .collect();

    Data { numbers, boards }
}

fn parse_block(i: usize, input: &str) -> Board {
    Board {
        i,
        grid: input.lines().map(parse_row).collect(),
    }
}

fn parse_row(input: &str) -> Vec<(u8, bool)> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|n| (n, false))
        .collect()
}

fn part_a(mut data: Data) -> u32 {
    for num in data.numbers {
        for board in &mut data.boards {
            board.mark(num);
            if board.check_win() {
                return num as u32 * board.sum_unmarked();
            }
        }
    }

    panic!("Got nada");
}

fn part_b(mut data: Data) -> u32 {
    let mut winners: HashSet<usize> = HashSet::new();
    let boards_len = data.boards.len();

    for num in data.numbers {
        for board in &mut data.boards {
            board.mark(num);
            if board.check_win() {
                winners.insert(board.i);
                if winners.len() == boards_len {
                    return num as u32 * board.sum_unmarked();
                }
            }
        }
    }

    panic!("Got nada");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let data = parse(input);
        assert_eq!(part_a(data), 4512);
    }

    #[test]
    fn part_two_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let data = parse(input);
        assert_eq!(part_b(data), 1924);
    }
}
