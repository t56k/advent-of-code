use std::fs;

#[derive(Debug)]
struct Operation {
    name: String,
    arg: isize,
}

pub fn main() {
    let contents = fs::read_to_string("./input").expect("Couldn't read input");

    let mut acc: isize = 0;
    let mut i: usize = 0;
    let mut ops: Vec<Operation> = parse_contents(contents);
    let mut ops_performed: Vec<usize> = [].to_vec();

    while !ops_performed.contains(&i) {
        ops_performed.push(i);
        calculate(&ops[i], &mut i, &mut acc);
    }
    println!("part one: {}", acc);

    let corruptions_to_check = get_possible_corruptions(&ops);
    corruptions_to_check
        .iter()
        .any(|i| test_corruption(&mut ops, *i));
}

fn read_lines(inp: &str) -> Vec<&str> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn parse_contents(inp: String) -> Vec<Operation> {
    read_lines(&inp)
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| parse_operation(line))
        .collect::<Vec<Operation>>()
}

fn parse_operation(line: &str) -> Operation {
    let args = line.split(' ').collect::<Vec<&str>>();
    let name = args[0].to_string();
    let arg = args[1].parse().unwrap();

    Operation { name, arg }
}

fn get_possible_corruptions(ops: &Vec<Operation>) -> Vec<usize> {
    ops.iter()
        .enumerate()
        .filter(|(_, op)| op.name == "nop" || op.name == "jmp")
        .map(|(corruption, _)| corruption)
        .collect()
}

fn test_corruption(ops: &mut Vec<Operation>, i: usize) -> bool {
    let old_op = ops[i].name.to_owned();
    ops[i].name = if old_op == "nop" {
        "jmp".to_string()
    } else {
        "nop".to_string()
    };

    let res = test_solve(&ops);
    ops[i].name = old_op.to_string();
    res
}

fn test_solve(ops: &Vec<Operation>) -> bool {
    let mut acc: isize = 0;
    let mut i: usize = 0;
    let mut ops_performed: Vec<usize> = [].to_vec();

    while !ops_performed.contains(&i) {
        if i == ops.len() {
            println!("part two: {}", acc);
            return true;
        }
        ops_performed.push(i);
        calculate(&ops[i], &mut i, &mut acc);
    }
    false
}

fn calculate(op: &Operation, i: &mut usize, acc: &mut isize) {
    let name = op.name.as_ref();
    let arg = op.arg;
    match name {
        "nop" => *i += 1,
        "acc" => {
            *i += 1;
            *acc += arg
        }
        "jmp" => {
            let new_i = (*i as isize) + arg;
            if new_i < 0 {
                panic!("jumping into negative space");
            }
            *i = new_i as usize;
        }
        invalid_op => panic!("invalid op {}", invalid_op),
    }
}
