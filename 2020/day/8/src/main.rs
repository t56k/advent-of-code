use std::fs;

#[derive(Debug)]
struct Operation {
    name: String,
    argument: isize,
}

pub fn main() {
    let acc: isize = 0;
    let ops_performed: Vec<usize> = [].to_vec();
    let contents = fs::read_to_string("./input").expect("Couldn't read input");
    let operations: Vec<Operation> = parse_contents(contents);

    perform(&operations, 0, acc, ops_performed);
}

fn perform(operations: &Vec<Operation>, i: usize, mut acc: isize, mut ops_performed: Vec<usize>) {
    if !ops_performed.iter().any(|&o| o == i) {
        ops_performed.push(i);

        let f = operations[i].name.as_ref();
        match f {
            "acc" => {
                acc += Some(operations[i].argument).unwrap();
                perform(operations, i + 1, acc, ops_performed)
            },
            "jmp" => perform(operations, add(i, operations[i].argument), acc, ops_performed),
            "nop" => perform(operations, i + 1, acc, ops_performed),
            _ => {
                println!("Invalid operation");
            }
        }
    } else {
       println!("{}", acc);
    }
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
    let argument = args[1].parse().unwrap();

    Operation { name, argument }
}

fn add(u: usize, i: isize) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
