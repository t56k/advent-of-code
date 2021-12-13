use std::collections::HashSet;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./input")
        .expect("Couldn't read the input");


    let groups = contents
        .split("\n\n")
        .collect::<Vec<_>>();

    let part_one_answers = groups.iter().map(|group| get_answers_in_group(group));
    let part_one: usize = part_one_answers.map(|a| a.len()).sum();

    println!("part one: {:?}", part_one);

    let part_two_answers = groups.iter().map(|group| get_common_answers_in_group(group));
    let part_two: usize = part_two_answers.map(|a| a.len()).sum();

    println!("part two: {:?}", part_two);
}

fn get_answers_in_group(group: &str) -> HashSet<char> {
    group
        .chars()
        .filter(|c| match c {
            'a'..='z' => true,
            _ => false,
        })
        .collect::<HashSet<char>>()
}

fn get_common_answers_in_group(group: &str) -> HashSet<char> {
    let people = group
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect::<Vec<HashSet<char>>>();

    if people.is_empty() {
        return HashSet::new();
    }

    let mut people_iter = people.iter();
    let mut common_answers = people_iter.next().unwrap().clone();
    people_iter.for_each(|answers| common_answers.retain(|x| answers.contains(x)));

    common_answers
}
