use std::fs;

const N: usize = 10;
const THRESHOLD: i8 = 10;
type Octopuses = [i8; N * N];
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("part one: {}", part_one.unwrap());
    println!("part two: {}", part_two.unwrap());
}

fn part_one(input: &str) -> Result<usize> {
    let octopuses = &mut parse_input(input)?;
    Ok((0..100).map(|_| next_step(octopuses)).sum())
}

fn part_two(input: &str) -> Result<usize> {
    let octopuses = &mut parse_input(input)?;
    (1..)
        .find(|_| next_step(octopuses) == N * N)
        .ok_or_else(|| "not found".into())
}

fn parse_input(input: &str) -> Result<Octopuses> {
    input
        .bytes()
        .filter(|&c| c.is_ascii_digit())
        .map(|c| (c - b'0') as i8)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| format!("not an array").into())
}

fn next_step(octopuses: &mut Octopuses) -> usize {
    octopuses.iter_mut().for_each(|x| match *x {
        -1 => *x = 1,
        _ => *x += 1,
    });
    (0..N * N)
        .filter_map(|pos| (octopuses[pos] >= THRESHOLD).then(|| flash(octopuses, pos)))
        .sum()
}

fn flash(octopuses: &mut Octopuses, pos: usize) -> usize {
    octopuses[pos] = -1;
    1 + [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .into_iter()
        .map(move |(di, dj)| (((pos / N) as isize + di) as usize, ((pos % N) as isize + dj) as usize))
        .filter(|&(ii, jj)| ii < N && jj < N)
        .map(|(ii, jj)| ii * N + jj)
        .filter_map(|pos| {
            (octopuses[pos] != -1)
                .then(|| octopuses[pos] += 1)
                .and((octopuses[pos] >= THRESHOLD).then(|| flash(octopuses, pos)))
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(part_one(&input).unwrap(), 1656);
    }

    #[test]
    fn part_two_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(part_two(&input).unwrap(), 517);
    }
}
