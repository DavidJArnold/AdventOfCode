use num::Integer;
use std::collections::HashMap;

const REAL_FILENAME: &str = "08.real.txt";

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.split('\n');

    let instructions = lines
        .next()
        .expect("Expects a line")
        .chars()
        .collect::<Vec<char>>();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();

    lines.next();

    for node in lines {
        let mut tags = node.split_whitespace();
        let source = tags.next().unwrap();
        let left = tags
            .nth(1)
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(',');
        let right = tags.next().unwrap().trim_end_matches(')');
        network.insert(source, (left, right));
    }

    (instructions, network)
}

fn part1(filename: &str) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.trim_end_matches('\n');
    let (instructions, network) = parse_input(input);
    let mut insts = instructions.iter().cycle();

    let mut pos: &str = "AAA";
    let mut counter = 0;
    while pos != "ZZZ" {
        counter += 1;
        pos = match insts.next().unwrap() {
            'L' => network.get(&pos).unwrap().0,
            'R' => network.get(&pos).unwrap().1,
            _ => panic!("Invalid instruction"),
        };
    }
    counter
}

fn part2(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (instructions, network) = parse_input(input);
    let mut insts = instructions.iter().cycle();

    let posses: Vec<&str> = network
        .iter()
        .map(|x| *x.0)
        .filter(|x| x.rfind('A') == Some(2))
        .collect::<Vec<_>>();
    let mut counts: Vec<u64> = vec![];
    for mut pos in posses {
        let mut counter: u64 = 0;
        while pos.rfind('Z') != Some(2) {
            counter += 1;
            pos = match insts.next().unwrap() {
                'L' => network.get(&pos).unwrap().0,
                'R' => network.get(&pos).unwrap().1,
                _ => panic!("Invalid instruction"),
            };
        }
        counts.push(counter);
    }
    counts.iter().fold(1, |acc, e| acc.lcm(e))
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME_1: &str = "08.test.1.txt";
    const TEST_FILENAME_2: &str = "08.test.2.txt";
    const TEST_FILENAME_3: &str = "08.test.3.txt";
    use crate::*;

    #[test]
    fn test_part1_1() {
        let ans1 = part1(&TEST_FILENAME_1);
        assert_eq!(ans1, 2)
    }

    #[test]
    fn test_part1_2() {
        let ans1 = part1(&TEST_FILENAME_2);
        assert_eq!(ans1, 6)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME_3);
        assert_eq!(ans2, 6)
    }
}
