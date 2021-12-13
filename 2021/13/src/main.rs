#![feature(stdio_locked)]
use std::{io::Write, iter::once};

const S: [usize; 2] = [40, 6];

// requires nightly: cargo +nightly run
pub fn main() {
    let (coords, folds) = include_str!("../input").split_once("\n\n").unwrap();
    let part_one = part_one(&coords, &folds);
    part_two(&coords, &folds);

    println!("part one: {}", part_one);
}

fn part_one(coords: &str, folds: &str) -> usize {
    let fold = folds.split_once('\n').unwrap().0;
    let (c, i) = fold[11..].split_once('=').unwrap();
    let (c, i) = (c.as_bytes()[0], i.parse().unwrap());

    let mut coords = coords
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
        .filter_map(|(x, y)| match c {
            b'x' if x == i => None,
            b'x' if x > i => Some((i - (x - i), y)),
            b'y' if y == i => None,
            b'y' if y > i => Some((x, i - (y - i))),
            _ => Some((x, y)),
        })
        .collect::<Vec<_>>();

    coords.sort_unstable();
    coords.dedup();
    coords.len()
}

fn part_two(coords: &str, folds: &str) {
    let folds = folds
        .lines()
        .map(|l| l.trim_start_matches("fold along ").split_once('=').unwrap())
        .map(|(c, i)| (c.as_bytes()[0], i.parse::<i16>().unwrap()))
        .collect::<Vec<_>>();

    std::io::stdout_locked()
        .write_all(
            &coords
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
                .filter_map(|(mut x, mut y)| {
                    for &(c, i) in &folds {
                        match c {
                            b'x' if x == i => return None,
                            b'x' if x > i => x = i - (x - i),
                            b'y' if y == i => return None,
                            b'y' if y > i => y = i - (y - i),
                            _ => {}
                        }
                    }
                    Some((x, y))
                })
                .fold(vec![0u64; S[1]], |mut map, (x, y)| {
                    map[y as usize] |= 1 << x;
                    map
                })
                .iter()
                .flat_map(|row| {
                    (0..S[0])
                        .map(move |x| b' ' + ((row & 1 << x) >> x) as u8 * 3)
                        .chain(once(b'\n'))
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let (coords, folds) = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            .split_once("\n\n")
            .unwrap();

        assert_eq!(part_one(&coords, &folds), 17);
    }
}
