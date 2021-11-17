use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Bag {
    colour: String,
    count: usize,
}

pub fn main() {
    let file = fs::read_to_string("./input").expect("Couldn't read the input");
    let rules = parse_bag_rules(file);
    let shiny_gold_holding_colours = get_shiny_gold_holding_bag_colours(&rules);

    println!("part one: {}", shiny_gold_holding_colours.len());
}

fn read_lines(inp: &str) -> Vec<&str> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn parse_bag_rules(inp: String) -> HashMap<String, Option<Vec<Bag>>> {
    read_lines(&inp)
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| parse_rule(line))
        .collect::<HashMap<String, Option<Vec<Bag>>>>()
}

fn parse_rule(line: &str) -> (String, Option<Vec<Bag>>) {
    let groups = line.split(" bags contain ");
    let vec: Vec<&str> = groups.collect();
    let colour = vec[0].to_owned();
    let contents: Option<Vec<Bag>> = vec[1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.split(" ").collect::<Vec<&str>>())
        .map(|value| match value[0] {
            "no" => None,
            _ => Some(create_bag(value[1], value[2], value[0])),
        })
        .collect();

    (colour, contents)
}

fn create_bag(colour_1: &str, colour_2: &str, count_str: &str) -> Bag {
    let mut colour = colour_1.to_owned();
    colour.push_str(" ");
    colour.push_str(colour_2);

    let count = count_str.parse::<usize>().unwrap();

    Bag { count, colour }
}

fn get_shiny_gold_holding_bag_colours(
    rules: &HashMap<String, Option<Vec<Bag>>>,
) -> HashSet<String> {
    let mut shiny_gold_holding_colours = HashSet::<String>::new();

    for rule in rules.iter() {
        if can_contain_shiny_gold_bag(&rule.0, rules, &mut shiny_gold_holding_colours) {
            shiny_gold_holding_colours.insert(rule.0.to_string());
        }
    }

    shiny_gold_holding_colours
}

fn can_contain_shiny_gold_bag(
    colour: &String,
    rules: &HashMap<String, Option<Vec<Bag>>>,
    shiny_gold_holding_colours: &mut HashSet<String>,
) -> bool {
    if shiny_gold_holding_colours.contains(colour) {
        return true;
    }

    let rule = &rules[colour];
    if rule.is_none() {
        return false;
    }

    match rule {
        None => {
            return false;
        }
        Some(bags) => {
            for inner_bag in bags.iter() {
                if is_shiny_gold_bag(inner_bag) {
                    return true;
                } else {
                    if can_contain_shiny_gold_bag(
                        &inner_bag.colour,
                        rules,
                        shiny_gold_holding_colours,
                    ) {
                        return true;
                    }
                }
            }
            return false;
        }
    }
}

fn is_shiny_gold_bag(bag: &Bag) -> bool {
    bag.colour == "shiny gold"
}
