use std::collections::HashSet;

const REAL_FILENAME: &str = "04.real.txt";

fn parse_input(input: &str) -> (Vec<HashSet<u32>>, Vec<HashSet<u32>>) {
    let mut winners: Vec<HashSet<u32>> = vec![];
    let mut ours: Vec<HashSet<u32>> = vec![];

    for line in input.split('\n') {
        let mut all_nums = line.split(": ").last().unwrap().split(" | ");
        winners.push(HashSet::from_iter(
            all_nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap()),
        ));
        ours.push(HashSet::from_iter(
            all_nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap()),
        ));
    }

    (winners, ours)
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap_or(&file_contents);
    let (winners, ours) = parse_input(input);
    let mut total: u32 = 0;
    for (win, us) in winners.iter().zip(ours.iter()) {
        let num_winners = win.intersection(us).count();
        if num_winners > 0 {
            total += 2_u32.pow(num_winners as u32 - 1);
        }
    }

    total
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap_or(&file_contents);
    let (winners, ours) = parse_input(input);
    let mut replicas = vec![];
    for _ in 0..ours.len() {
        replicas.push(1);
    }
    for idx in 0..ours.len() {
        let num_winners = winners
            .get(idx)
            .unwrap()
            .intersection(ours.get(idx).unwrap())
            .count();
        for kdx in 0..num_winners {
            replicas[idx + 1 + kdx] += replicas[idx];
        }
    }

    replicas.iter().sum()
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
    const TEST_FILENAME: &str = "04.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 13)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 30)
    }
}
