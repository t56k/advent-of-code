use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl From<&str> for Cuboid {
    fn from(s: &str) -> Self {
        let c = s
            .split(',')
            .flat_map(|p| {
                p.split('=')
                    .nth(1)
                    .unwrap()
                    .split("..")
                    .map(|v| v.parse::<i32>().unwrap())
            })
            .collect::<Vec<_>>();
        Cuboid::new(c[0], c[1], c[2], c[3], c[4], c[5])
    }
}

impl Cuboid {
    pub fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        Cuboid {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    pub fn intersect_split(&mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut result_vec = Vec::new();
        if (self.x_min <= other.x_max && self.x_max >= other.x_min)
            && (self.y_min <= other.y_max && self.y_max >= other.y_min)
            && (self.z_min <= other.z_max && self.z_max >= other.z_min)
        {
            // on x
            if self.x_min < other.x_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    other.x_min - 1,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.x_min = other.x_min;
            }
            if self.x_max > other.x_max {
                result_vec.push(Cuboid::new(
                    other.x_max + 1,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.x_max = other.x_max;
            }
            // on y
            if self.y_min < other.y_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    other.y_min - 1,
                    self.z_min,
                    self.z_max,
                ));
                self.y_min = other.y_min;
            }
            if self.y_max > other.y_max {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    other.y_max + 1,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.y_max = other.y_max;
            }
            // on z
            if self.z_min < other.z_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    other.z_min - 1,
                ));
                self.z_min = other.z_min;
            }
            if self.z_max > other.z_max {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    other.z_max + 1,
                    self.z_max,
                ));
                self.z_max = other.z_max;
            }
        } else {
            result_vec.push(*self)
        }
        result_vec
    }

    pub fn area(&self) -> usize {
        let mut result = 1;
        result *= (self.x_max - self.x_min) as usize + 1;
        result *= (self.y_max - self.y_min) as usize + 1;
        result *= (self.z_max - self.z_min) as usize + 1;
        result
    }
}

fn parse(input: &str) -> Vec<Cuboid> {
    input.lines().fold(Vec::<Cuboid>::new(), |mut acc, line| {
        let (switch_str, coords) = line.split_once(' ').unwrap();
        let switch = switch_str == "on";
        let mut cuboids = Vec::with_capacity(acc.len() + 24);
        let parsed_cuboid = Cuboid::from(coords);
        for oc in acc.iter_mut() {
            cuboids.append(&mut oc.intersect_split(&parsed_cuboid));
        }
        if switch {
            cuboids.push(parsed_cuboid);
        }
        cuboids
    })
}

fn part_one(v: &[Cuboid]) -> usize {
    v.iter()
        .filter(|c| {
            c.x_min >= -50
                && c.x_max <= 50
                && c.y_min >= -50
                && c.y_max <= 50
                && c.z_min >= -50
                && c.z_max <= 50
        })
        .map(|c| c.area())
        .sum()
}

fn part_two(v: &[Cuboid]) -> usize {
    v.iter().map(|c| c.area()).sum()
}

fn main() {
    let input = include_str!("../input");
    let cubes = parse(&input);

    benchmark(|| println!("part one: {}", part_one(&cubes)));
    benchmark(|| println!("part two: {}", part_two(&cubes)));
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
    let cubes = parse(&input);

    assert_eq!(590784, part_one(&cubes));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example_two");
    let cubes = parse(&input);

    assert_eq!(2758514936282235, part_two(&cubes));
}
