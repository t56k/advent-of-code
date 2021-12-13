use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn part_one(input: &str) -> i64 {
    let mut total = 0;

    'next_line: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.len() == 0 || c != closing_brace(*stack.last().unwrap()) {
                        total += score(c);
                        continue 'next_line;
                    }
                    stack.pop();
                }
                _ => panic!("unrecognized character {}", c),
            }
        }
    }

    total
}

fn part_two(input: &str) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    'next_line: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.len() == 0 || c != closing_brace(*stack.last().unwrap()) {
                        continue 'next_line;
                    }
                    stack.pop();
                }
                _ => panic!("unrecognized character {}", c),
            }
        }

        stack.reverse();
        let linescore = stack
            .iter()
            .fold(0, |accum, c| (accum * 5) + score_two(closing_brace(*c)));

        scores.push(linescore);
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unrecognized character {}", c),
    }
}

fn score_two(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unrecognized character {}", c),
    }
}

fn closing_brace(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unrecognized character {}", c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part_one(&input), 26397);
    }

    #[test]
    fn part_two_example() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part_two(&input), 288957);
    }
}
