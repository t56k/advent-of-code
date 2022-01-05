use std::time::Instant;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

fn translate(p: &Point, dx: i64, dy: i64) -> Point {
    Point::new(p.x + dx, p.y + dy)
}

fn rotate(p: &Point, deg: i64) -> Point {
    match deg {
        0 => Point::new(p.x, p.y),
        90 => Point::new(p.y, -p.x),
        180 => Point::new(-p.x, -p.y),
        270 => Point::new(-p.y, p.x),
        _ => unreachable!(),
    }
}

fn manhattan(c: &Point) -> i64 {
    c.x.abs() + c.y.abs()
}

fn part_one(input: &str) -> i64 {
    let (ship, _) = input
        .lines()
        .into_iter()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .fold(
            (Point::new(0, 0), Point::new(1, 0)),
            |(ship, waypoint), (cmd, arg)| match cmd {
                'N' => (translate(&ship, 0, arg), waypoint),
                'S' => (translate(&ship, 0, -arg), waypoint),
                'E' => (translate(&ship, arg, 0), waypoint),
                'W' => (translate(&ship, -arg, 0), waypoint),
                'L' => (ship, rotate(&waypoint, 360 - arg)),
                'R' => (ship, rotate(&waypoint, arg)),
                'F' => (
                    translate(&ship, waypoint.x * arg, waypoint.y * arg),
                    waypoint,
                ),
                _ => unreachable!(),
            },
        );
    Some(manhattan(&ship)).unwrap()
}

fn part_two(input: &str) -> i64 {
    let (ship, _) = input
        .lines()
        .into_iter()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .fold(
            (Point::new(0, 0), Point::new(10, 1)),
            |(ship, waypoint), (cmd, arg)| match cmd {
                'N' => (ship, translate(&waypoint, 0, arg)),
                'S' => (ship, translate(&waypoint, 0, -arg)),
                'E' => (ship, translate(&waypoint, arg, 0)),
                'W' => (ship, translate(&waypoint, -arg, 0)),
                'L' => (ship, rotate(&waypoint, 360 - arg)),
                'R' => (ship, rotate(&waypoint, arg)),
                'F' => (
                    translate(&ship, waypoint.x * arg, waypoint.y * arg),
                    waypoint,
                ),
                _ => unreachable!(),
            },
        );
    Some(manhattan(&ship)).unwrap()
}

fn main() {
    let input = include_str!("../input");

    benchmark(|| println!("part one: {}", part_one(&input)));
    benchmark(|| println!("part two: {}", part_two(&input)));
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
    let input = include_str!("../example_one");

    assert_eq!(25, part_one(&input));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");

    assert_eq!(286, part_two(&input));
}
