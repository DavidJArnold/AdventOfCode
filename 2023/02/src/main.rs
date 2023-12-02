const REAL_FILENAME: &str = "02.real.txt";

fn parse_input(input: &str) -> Vec<Vec<Vec<u32>>> {
    let mut games: Vec<Vec<Vec<u32>>> = vec![];
    for line in input.split('\n') {
        let mut rounds: Vec<Vec<u32>> = vec![];
        for subset in line.split(':').nth(1).unwrap().trim().split(';') {
            let mut counts = vec![0, 0, 0];
            for colour_group in subset.trim().split(',') {
                let mut group = colour_group.trim().split(' ');
                let num = group.next().unwrap();
                let colour = group.next().unwrap();
                match colour {
                    "red" => counts[0] += num.parse::<u32>().unwrap(),
                    "green" => counts[1] += num.parse::<u32>().unwrap(),
                    "blue" => counts[2] += num.parse::<u32>().unwrap(),
                    _ => panic!("unknown option"),
                }
            }
            rounds.push(counts);
        }
        games.push(rounds);
    }
    games
}

fn part1(filename: &str) -> usize {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    let total_cubes = vec![12, 13, 14];
    let mut result: usize = 0;
    for (idx, game) in data.iter().enumerate() {
        let mut valid = true;
        for colour in 0..3 {
            if total_cubes[colour] < game.iter().map(|x| x[colour]).max().unwrap() {
                valid = false
            }
        }
        if valid {
            result += idx + 1_usize
        }
    }

    result
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    let mut total = 0;
    for game in data {
        let mut power = 1;
        for colour in 0..3 {
            power *= game.iter().map(|x| x[colour]).max().unwrap();
        }
        total += power;
    }
    total
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
    const TEST_FILENAME: &str = "02.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 8)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 2286)
    }
}
