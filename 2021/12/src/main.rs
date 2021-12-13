use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Cave {
    connected: Vec<usize>,
    large: bool,
}

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn part_one(input: &str) -> u16 {
    let mut cave_indexes = HashMap::new();
    let mut caves = Vec::new();

    for line in input.lines() {
        let words: Vec<_> = line.split('-').collect();
        for &word in &words {
            if !cave_indexes.contains_key(word) {
                cave_indexes.insert(word, caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: word.chars().next().unwrap().is_ascii_uppercase(),
                });
            }
        }

        let a = cave_indexes[words[0]];
        let b = cave_indexes[words[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = cave_indexes["start"];
    let end = cave_indexes["end"];

    count_paths_for_part_one(&caves, &mut vec![false; caves.len()], start, end)
}

fn part_two(input: &str) -> u32 {
    let mut cave_indexes = HashMap::new();
    let mut caves = Vec::new();

    for line in input.lines() {
        let words: Vec<_> = line.split('-').collect();
        for &word in &words {
            if !cave_indexes.contains_key(word) {
                cave_indexes.insert(word, caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: word.chars().next().unwrap().is_ascii_uppercase(),
                });
            }
        }

        let a = cave_indexes[words[0]];
        let b = cave_indexes[words[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = cave_indexes["start"];
    let end = cave_indexes["end"];

    count_paths_for_part_two(&caves, &mut vec![0; caves.len()], false, start, start, end)
}

fn count_paths_for_part_one(caves: &Vec<Cave>, visited: &mut Vec<bool>, pos: usize, end: usize) -> u16 {
    let cave = &caves[pos];

    if pos == end {
        1
    } else if visited[pos] && !cave.large {
        0
    } else {
        visited[pos] = true;
        let paths = cave.connected.iter().map(|&new_pos| {
            count_paths_for_part_one(caves, visited, new_pos, end)
        }).sum();
        visited[pos] = false;
        paths
    }
}

fn count_paths_for_part_two(caves: &Vec<Cave>, visited: &mut Vec<u8>, multi_visited: bool, pos: usize, start: usize, end: usize) -> u32 {
    let cave = &caves[pos];
    let second_visit = visited[pos] >= 1 && !cave.large;

    if pos == end {
        1
    } else if second_visit && (multi_visited || pos == start) {
        0
    } else {
        let multi_visited = multi_visited || second_visit;
        visited[pos] += 1;
        let paths = cave.connected.iter().map(|&new_pos| {
            count_paths_for_part_two(caves, visited, multi_visited, new_pos, start, end)
        }).sum();
        visited[pos] -= 1;
        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(part_one(&input), 10);
    }
}
