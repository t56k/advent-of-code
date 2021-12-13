use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    str::FromStr,
};

#[derive(Debug)]
struct Vent {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Vent {
    fn new(values: &Vec<u32>) -> Self {
        let x1 = values[0];
        let y1 = values[1];
        let x2 = values[2];
        let y2 = values[3];

        Self { x1, y1, x2, y2 }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn is_diagonal(&self) -> bool {
        i32::abs(self.x2 as i32 - self.x1 as i32) == i32::abs(self.y2 as i32 - self.y1 as i32)
    }
}

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let parsed_input = parse(&input);
    let part_one = part_one(&parsed_input);
    let part_two = part_two(&parsed_input);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn parse(input: &str) -> Vec<Vent> {
    input
        .replace(" -> ", ",")
        .lines()
        .map(|l| Vent::new(&l.split(",").map(|v| u32::from_str(v).unwrap()).collect()))
        .collect()
}

fn part_one(input: &Vec<Vent>) -> u32 {
    let mut hits: HashMap<(u32, u32), u32> = HashMap::new();

    for v in input.iter().filter(|v| v.is_horizontal_or_vertical()) {
        for x in min(v.x1, v.x2)..=max(v.x1, v.x2) {
            for y in min(v.y1, v.y2)..=max(v.y1, v.y2) {
                let l = hits.entry((x, y)).or_insert(0);
                *l += 1;
            }
        }
    }

    hits.values().filter(|v| **v > 1).count() as u32
}

fn part_two(input: &Vec<Vent>) -> u32 {
    let mut hits: HashMap<(u32, u32), u32> = HashMap::new();

    for v in input.iter().filter(|v| v.is_horizontal_or_vertical()) {
        for x in min(v.x1, v.x2)..=max(v.x1, v.x2) {
            for y in min(v.y1, v.y2)..=max(v.y1, v.y2) {
                let l = hits.entry((x, y)).or_insert(0);
                *l += 1;
            }
        }
    }

    for v in input.iter().filter(|v| v.is_diagonal()) {
		let mut x = v.x1 as i32;
		let mut y = v.y1 as i32;
		let x_i = if v.x1 >= v.x2 { -1 } else { 1 };
		let y_i = if v.y1 >= v.y2 { -1 } else { 1 };
		while v.x2 as i32 != (x - x_i) {
			let l = hits.entry((x as u32, y as u32)).or_insert(0);
			*l += 1;
			x += x_i;
			y += y_i;
		}
	}

    hits.values().filter(|v| **v > 1).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let data = parse(input);
        assert_eq!(part_one(&data), 5);
    }

    #[test]
    fn part_two_example() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let data = parse(input);
        assert_eq!(part_two(&data), 12);
    }
}
