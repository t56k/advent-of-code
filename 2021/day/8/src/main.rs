use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input").expect("Couldn't read input");
    let parsed_input = parse(&input);

    let part_one = part_one(&parsed_input);
    let part_two = part_two(&parsed_input);

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);
}

fn parse(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" | ");
            let patterns = it
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| SignalPattern::from(s))
                .collect();
            let output = it
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| SignalPattern::from(s))
                .collect();
            Display::new(patterns, output)
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct SignalPattern {
    pattern: Vec<u8>,
}

impl SignalPattern {
    fn number_of_intersect(&self, other: &Self) -> usize {
        self.pattern
            .iter()
            .filter(|p| other.pattern.contains(p))
            .count()
    }
}

impl From<&str> for SignalPattern {
    fn from(s: &str) -> Self {
        let mut v = s.chars().map(|c| c as u8 - b'a').collect::<Vec<_>>();
        v.sort();
        Self { pattern: v }
    }
}

struct Display {
    patterns: Vec<SignalPattern>,
    output: Vec<SignalPattern>,
}

#[inline]
fn filter_patterns_get_first<'a>(
    patterns: &'a Vec<SignalPattern>,
    filter: &dyn Fn(&'a SignalPattern) -> bool,
) -> &'a SignalPattern {
    patterns.iter().filter(|p| filter(p)).next().unwrap()
}

#[inline]
fn filter_pattern_value<'a>(pattern: &'a SignalPattern, filter: &dyn Fn(u8) -> bool) -> u8 {
    pattern
        .pattern
        .iter()
        .copied()
        .filter(|&x| filter(x))
        .next()
        .unwrap()
}

impl Display {
    fn decode(&self) -> usize {
        // known 4 assignments are 1 4 7 8
        let one = filter_patterns_get_first(&self.patterns, &|p| p.pattern.len() == 2);
        let four = filter_patterns_get_first(&self.patterns, &|p| p.pattern.len() == 4);
        let seven = filter_patterns_get_first(&self.patterns, &|p| p.pattern.len() == 3);
        let eight = filter_patterns_get_first(&self.patterns, &|p| p.pattern.len() == 7);

        // 6 has len 6 and 1 intersection with 1 (0 and 9 has 2 intersection)
        let six = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 6 && p.number_of_intersect(one) == 1
        });

        // 6 and 1 intersection is f
        let f_value = filter_pattern_value(six, &|x| one.pattern.contains(&x));

        // 1 - f is c
        let c_value = filter_pattern_value(one, &|x| x != f_value);

        // 3 has len 5 and has c and f (2 and 5 don't have c and f at the same time)
        let three = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 5 && p.pattern.contains(&c_value) && p.pattern.contains(&f_value)
        });

        // 2 has len 5 and 2 intersections with 4 (3 and 5 has 3 intersect with 4)
        let two = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 5 && four.number_of_intersect(p) == 2
        });

        // 4 - 3 gives b
        let b_value = filter_pattern_value(four, &|x| !three.pattern.contains(&x));

        // 5 has len 5 and contains b (2 and 3 don't have b)
        let five = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 5 && p.pattern.contains(&b_value)
        });

        // 4 - 1 - b gives d
        let d_value = filter_pattern_value(four, &|x| !one.pattern.contains(&x) && x != b_value);

        // 0 has len 6 and does not have d (6 and 9 have d)
        let zero = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 6 && !p.pattern.contains(&d_value)
        });

        // 9 has len 6 and contains c and d (0 doesn't have d, 6 doesn't have c)
        let nine = filter_patterns_get_first(&self.patterns, &|p| {
            p.pattern.len() == 6 && p.pattern.contains(&d_value) && p.pattern.contains(&c_value)
        });

        let v = [zero, one, two, three, four, five, six, seven, eight, nine];

        self.output
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| v.iter().position(|&s| s == x).unwrap() * 10usize.pow(i as u32))
            .sum()
    }
}

impl Display {
    fn new(patterns: Vec<SignalPattern>, output: Vec<SignalPattern>) -> Self {
        Self { patterns, output }
    }
}

fn part_one(v: &[Display]) -> usize {
    v.iter()
        .flat_map(|d| &d.output)
        .map(|s| &s.pattern)
        .filter(|v| v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7)
        .count()
}

fn part_two(v: &[Display]) -> usize {
    v.iter().map(|x| x.decode()).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let parsed_input = parse(&input);
        assert_eq!(part_one(&parsed_input), 26);
    }

    #[test]
    fn part_two_example() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let parsed_input = parse(&input);
        assert_eq!(part_two(&parsed_input), 61229);
    }
}
