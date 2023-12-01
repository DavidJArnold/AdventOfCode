use serde::Deserialize;
use serde_json;
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum VecEntry {
    Number(u8),
    Vector(Vec<VecEntry>),
}
impl PartialEq for VecEntry {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
impl Eq for VecEntry {}
impl PartialOrd for VecEntry {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for VecEntry {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        // This is important
        match (self, rhs) {
            (Self::Number(l), Self::Number(r)) => l.cmp(r),
            (Self::Vector(l), Self::Vector(r)) => l.cmp(r),
            (Self::Number(l), Self::Vector(r)) => [VecEntry::Number(*l)][..].cmp(r),
            (Self::Vector(l), Self::Number(r)) => l.as_slice().cmp(&[VecEntry::Number(*r)]),
        }
    }
}

fn read_input(contents: &str) -> Vec<Vec<VecEntry>> {
    contents
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|x| x.split("\n").map(|y| parse_vec(y)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn parse_vec(string_vec: &str) -> VecEntry {
    let parsed_vec = serde_json::from_str::<VecEntry>(&string_vec).unwrap();
    return parsed_vec;
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = read_input(&file_contents);
    let mut output: usize = 0;
    for (idx, group) in input.iter().enumerate() {
        if group[0] < group[1] {
            output += idx + 1;
        }
    }
    return output as u32;
}

fn part2(filename: &str) -> usize {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = read_input(&file_contents);
    let mut all_groups: Vec<VecEntry> = vec![];
    for group in input.iter() {
        all_groups.extend_from_slice(&group);
    }
    all_groups.push(VecEntry::Vector(vec![VecEntry::Number(2)]));
    all_groups.push(VecEntry::Vector(vec![VecEntry::Number(6)]));
    all_groups.sort();
    let mut output: usize = 1;
    for (idx, el) in all_groups.iter().enumerate() {
        if *el == VecEntry::Vector(vec![VecEntry::Number(2)])
            || *el == VecEntry::Vector(vec![VecEntry::Number(6)])
        {
            output *= idx + 1;
        }
    }
    return output;
}

fn main() {
    let filename = "13.real.txt";
    let ans1 = part1(&filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(&filename);
    println!("Part 2: {}", ans2);
}
