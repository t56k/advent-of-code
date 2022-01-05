use std::time::Instant;

const COL: isize = 98;
const ROW: isize = 97;

const NONE: u8 = 0;
const EMPTY: u8 = 1;
const FULL: u8 = 2;

fn parse_input() -> Vec<usize> {
    include_bytes!("../input")
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s == &&b'L')
        .map(|(i, _)| i - (i / (COL as usize + 1)))
        .collect()
}

fn part_one(seats: &Vec<usize>) -> usize {
    let mut cur = [NONE; (ROW * COL) as usize];

    for i in seats {
        cur[*i] = EMPTY;
    }

    let mut prev = cur;

    let seats: Vec<(usize, Vec<usize>)> = seats
        .iter()
        .map(|i| {
            (
                *i,
                (0..9)
                    .filter(|r| r != &4)
                    .map(|r| {
                        (
                            (*i as isize % COL) + r % 3 - 1,
                            (*i as isize / COL) + r / 3 - 1,
                        )
                    })
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < COL && *y < ROW)
                    .map(|(x, y)| (y * COL + x) as usize)
                    .filter(|i| cur[*i] == EMPTY)
                    .collect(),
            )
        })
        .collect();

    while {
        for (i, visible) in &seats {
            let occup = visible.iter().filter(|i| prev[**i] == FULL).count();
            let (cur_seat, prev_seat) = (&mut cur[*i], prev[*i]);

            if prev_seat == EMPTY && occup == 0 {
                *cur_seat = FULL;
            } else if prev_seat == FULL && occup >= 4 {
                *cur_seat = EMPTY;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);
        cur != prev
    } {}

    cur.iter().filter(|s| s == &&FULL).count()
}

fn part_two(seats: &Vec<usize>) -> usize {
    let mut cur = [NONE; (ROW * COL) as usize];

    for i in seats {
        cur[*i] = EMPTY;
    }

    let mut prev = cur;

    let seats: Vec<(usize, Vec<usize>)> = seats
        .clone()
        .into_iter()
        .map(|i| (i, (0..9)
            .filter(|r| r != &4)
            .map(|r| (r % 3 - 1, r / 3 - 1))
            .filter_map(|(rx, ry)| (1..)
                .map(|f| ((i as isize % COL) + rx * f, (i as isize / COL) + ry * f))
                .take_while(|(x, y)| *x >= 0 && *y >= 0 && *x < COL && *y < ROW)
                .map(|(x, y)| (y * COL + x) as usize)
                .filter(|i| cur[*i] == EMPTY)
                .next()
            )
            .collect(),
        ))
        .collect::<Vec<_>>();

    while {
        for (i, visible) in &seats {
            let occup = visible.iter().filter(|i| prev[**i] == FULL).count();
            let (cur_seat, prev_seat) = (&mut cur[*i], prev[*i]);

            if prev_seat == EMPTY && occup == 0 {
                *cur_seat = FULL;
            } else if prev_seat == FULL && occup >= 5 {
                *cur_seat = EMPTY;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);
        cur != prev
    } {}

    cur.iter().filter(|s| s == &&FULL).count()
}

fn main() {
    let seats = parse_input();

    benchmark(|| println!("part one: {}", part_one(&seats)));
    benchmark(|| println!("part two: {}", part_two(&seats)));
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
    let seats = include_bytes!("../example_one")
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s == &&b'L')
        .map(|(i, _)| i - (i / (COL as usize + 1)))
        .collect();

    assert_eq!(37, part_one(&seats));
}

#[test]
fn check_part_two() {
    let seats = include_bytes!("../example_two")
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s == &&b'L')
        .map(|(i, _)| i - (i / (COL as usize + 1)))
        .collect();

    assert_eq!(26, part_two(&seats));
}

