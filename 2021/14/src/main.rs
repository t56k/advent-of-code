use std::mem;

pub fn main() {
    let (polymer, insertions) = include_str!("../input").split_once("\n\n").unwrap();

    let mut insertions = insertions
        .lines()
        .map(|l| {
            let (k, t) = l.split_once(" -> ").unwrap();
            let (k, t) = (k.as_bytes(), t.as_bytes()[0]);
            ([k[0], k[1]], [k[0], t])
        })
        .collect::<Vec<_>>();
    insertions.sort_unstable_by_key(|r| r.0);

    let insertion = insertions
        .iter()
        .map(|r| {
            (
                r.0,
                insertions.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                insertions
                    .binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let part_one = process(&polymer, insertion.clone(), 10);
    let part_two = process(&polymer, insertion, 40);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn process(polymer: &str, insertion: Vec<([u8; 2], usize, usize)>, count: u64) -> u64 {
    let polymer = polymer.as_bytes().to_vec();
    let (mut num, mut next) = (vec![0; insertion.len()], vec![0; insertion.len()]);

    polymer
        .windows(2)
        .for_each(|key| num[insertion.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);

    (0..count).for_each(|_| {
        num.iter_mut().zip(&insertion).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occur = [0; (b'Z' - b'A') as usize];
    occur[(polymer.last().unwrap() - b'A') as usize] += 1;
    insertion
        .iter()
        .zip(num)
        .for_each(|(r, n)| occur[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occur
        .iter()
        .filter(|&&n| n != 0)
        .fold((u64::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));

    max - min
}
