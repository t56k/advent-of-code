use std::collections::HashMap;
use std::time::Instant;

fn parse_positions(input: &str) -> (i32, i32) {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth_back(0)
            .unwrap()
            .parse()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth_back(0)
            .unwrap()
            .parse()
            .unwrap(),
    )
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Player {
    score: i32,
    location: i32,
}

impl Player {
    fn new(location: i32) -> Self {
        Self { score: 0, location }
    }

    fn advance(&mut self, amount: i32) {
        self.location += amount % 10;
        if self.location > 10 {
            self.location -= 10
        }
        self.score += self.location;
    }
}

fn part_one(input: &str) -> i32 {
    let (a, b) = parse_positions(input);

    let mut die = (1..=100).cycle();

    let mut player_one = Player::new(a);
    let mut player_two = Player::new(b);
    let mut rolls = 0;

    while player_one.score < 1000 && player_two.score < 1000 {
        rolls += 3;
        let total = (0..3).flat_map(|_| die.next()).sum();
        player_one.advance(total);
        std::mem::swap(&mut player_one, &mut player_two);
    }

    rolls * player_one.score.min(player_two.score)
}

#[derive(Clone, Copy)]
struct Scores(i128, i128);

impl std::ops::Add<Scores> for Scores {
    type Output = Self;

    fn add(self, other: Self) -> <Self as std::ops::Add<Self>>::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::AddAssign<Scores> for Scores {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Scores {
    fn max(&self) -> i128 {
        self.0.max(self.1)
    }

    fn reverse(&self) -> Self {
        Scores(self.1, self.0)
    }
}

fn play(players: (Player, Player), cache: &mut HashMap<(Player, Player), Scores>) -> Scores {
    let (a, b) = players;

    if a.score >= 21 {
        return Scores(1, 0);
    }

    if b.score >= 21 {
        return Scores(0, 1);
    }

    if let Some(&value) = cache.get(&players) {
        return value;
    }

    let mut running_total = Scores(0, 0);
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let (mut a, _) = players;
                a.advance(i + j + k);
                running_total += play((b, a), cache).reverse();
            }
        }
    }

    cache.insert(players, running_total);

    running_total
}

fn part_two(input: &str) -> i128 {
    let (a, b) = parse_positions(input);

    let player_one = Player::new(a);
    let player_two = Player::new(b);

    let mut cache = HashMap::new();

    play((player_one, player_two), &mut cache).max()
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

fn main() {
    let input = include_str!("../input");

    benchmark(|| println!("part one: {}", part_one(&input)));
    benchmark(|| println!("part two: {}", part_two(&input)));
}

#[test]
fn check_part_one() {
    let input = include_str!("../example");
    assert_eq!(739785, part_one(&input));
}

#[test]
fn check_part_two() {
    let input = include_str!("../example");
    assert_eq!(444356092776315, part_two(&input));
}
