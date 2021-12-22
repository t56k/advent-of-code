use std::cmp::{max, min};
use std::collections::*;
use std::time::Instant;

type Range = std::ops::RangeInclusive<isize>;
type CubeRange = Vec<(bool, Range, Range, Range)>;

fn parse(data: &str) -> CubeRange {
    data.lines()
        .map(|line| {
            let (state, ranges) = line.split_once(' ').unwrap();
            let state = if state == "on" {
                true
            } else if state == "off" {
                false
            } else {
                unreachable!();
            };

            let mut xyz = ranges.split(',').map(|rng| {
                let rng = rng.split_once('=').unwrap().1;
                let mut rng = rng.split("..").map(|x| x.parse().unwrap());
                let a = rng.next().unwrap();
                let b = rng.next().unwrap();
                a..=b
            });

            (
                state,
                xyz.next().unwrap(),
                xyz.next().unwrap(),
                xyz.next().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn part_one(cube_ranges: &CubeRange) -> isize {
    let mut on_cubes = HashSet::new();
    for cube in cube_ranges.iter() {
        on_cubes = calc_part_one(&on_cubes, cube);
    }

    on_cubes.len() as isize
}

fn calc_part_one(
    on_cubes: &HashSet<(isize, isize, isize)>,
    instr: &(bool, Range, Range, Range),
) -> HashSet<(isize, isize, isize)> {
    let mut res = on_cubes.clone();
    let (state, xr, yr, zr) = instr;

    for x in max(-50, *xr.start())..=min(50, *xr.end()) {
        for y in max(-50, *yr.start())..=min(50, *yr.end()) {
            for z in max(-50, *zr.start())..=min(50, *zr.end()) {
                if *state {
                    res.insert((x, y, z));
                } else {
                    res.remove(&(x, y, z));
                }
            }
        }
    }

    res
}

fn main() {
    let input = include_str!("../input");
    let cube_ranges = parse(input);
    let part_one = part_one(&cube_ranges);

    benchmark(|| println!("part one: {}", part_one));
}

fn benchmark<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = f();
    println!("time: {}µs", start.elapsed().as_micros());

    result
}

#[test]
fn check_part_one() {
    let input = include_str!("../example");
    let cube_ranges = parse(input);

    assert_eq!(590784, part_one(&cube_ranges));
}
