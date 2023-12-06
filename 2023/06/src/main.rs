const REAL_FILENAME: &str = "06.real.txt";

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

fn parse_input_part_1(input: &str) -> Vec<Race> {
    let race_info: Vec<Vec<u64>> = input
        .split('\n')
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .map(|y| y.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    race_info[0]
        .iter()
        .zip(race_info[1].iter())
        .map(|(a, b)| Race {
            time: *a,
            record: *b,
        })
        .collect()
}

fn parse_input_part_2(input: &str) -> Race {
    let race_info: Vec<String> = input
        .split('\n')
        .map(|x| {
            x.split_whitespace()
                .skip(1)
                .fold("".to_string(), |x, y| x + y)
        })
        .collect();
    Race {
        time: race_info[0].parse::<u64>().unwrap(),
        record: race_info[1].parse::<u64>().unwrap(),
    }
}

fn run_race(race: &Race) -> u64 {
    let min_win =
        ((race.time as f64 - ((race.time.pow(2) - 4 * race.record) as f64).sqrt()) / 2.0).ceil();
    let max_win =
        ((race.time as f64 + ((race.time.pow(2) - 4 * race.record) as f64).sqrt()) / 2.0).floor();
    // since we're checking for roots, we need to exclude the case where we only tie the record
    if min_win as u64 * (race.time - min_win as u64) == race.record
        || max_win as u64 * (race.time - max_win as u64) == race.record
    {
        (max_win - min_win - 1.0) as u64
    } else {
        (max_win - min_win + 1.0) as u64
    }
}

fn part1(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let races = parse_input_part_1(input);
    races
        .iter()
        .map(|x| run_race(x))
        .reduce(|acc, e| acc * e)
        .unwrap() as u64
}

fn part2(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let race = parse_input_part_2(input);
    run_race(&race) as u64
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
    const TEST_FILENAME: &str = "06.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 288)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 71503)
    }
}
