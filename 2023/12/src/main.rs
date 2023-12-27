use memoize::memoize;

const REAL_FILENAME: &str = "12.real.txt";

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Record {
    springs: Vec<Condition>,
    pattern: Vec<u64>,
}

fn parse_spring_string(springs: &str) -> Vec<Condition> {
    springs
        .chars()
        .map(|x| match x {
            '?' => Condition::Unknown,
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            x => panic!("Unknown spring Condition character {x}"),
        })
        .collect()
}

fn _get_spring_string(springs: &[Condition]) -> String {
    springs
        .iter()
        .map(|x| match x {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        })
        .collect::<String>()
}

fn parse_input(input: &str) -> Vec<Record> {
    let mut output: Vec<Record> = vec![];
    for line in input.strip_suffix('\n').unwrap_or(input).lines() {
        let mut line_parts = line.split_ascii_whitespace();
        let conditions: Vec<Condition> = parse_spring_string(line_parts.next().unwrap());

        let pattern: Vec<_> = line_parts
            .next()
            .unwrap()
            .split(',')
            .filter_map(|x| {
                if x == "," {
                    None
                } else {
                    Some(x.parse::<u64>().unwrap())
                }
            })
            .collect();
        output.push(Record {
            springs: conditions,
            pattern,
        });
    }
    output
}

fn dot(state: Record) -> u64 {
    let mut right = state;
    right.springs = right.springs[1..].to_vec();
    solve(right)
}

fn pound(left: Record) -> u64 {
    let next_group = left.pattern[0];
    let this_group = left.springs.clone();
    if next_group as usize > this_group.len() {
        return 0;
    }
    if this_group[0..next_group as usize]
        .iter()
        .any(|x| x == &Condition::Operational)
    {
        return 0;
    }

    if left.springs.len() == next_group as usize {
        if left.pattern.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    if left.springs[next_group as usize] != Condition::Damaged {
        return solve(Record {
            pattern: left.pattern[1..].to_vec(),
            springs: left.springs[next_group as usize + 1..].to_vec(),
        });
    }
    0
}

#[memoize]
fn solve(input: Record) -> u64 {
    if input.pattern.is_empty() {
        if input.springs.is_empty() || !input.springs.contains(&Condition::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }

    if input.springs.is_empty() {
        return 0;
    }

    match input.springs[0] {
        Condition::Operational => dot(input),
        Condition::Damaged => pound(input),
        Condition::Unknown => dot(input.clone()) + pound(input),
    }
}

fn call_solve(data: &[Record], n_repeats: usize) -> u64 {
    let mut total = 0;
    for x in data.iter() {
        let mut springs = x.springs.clone();
        for _ in 1..n_repeats {
            springs.push(Condition::Unknown);
            springs.extend(x.springs.clone());
        }
        let case = Record {
            springs,
            pattern: x.pattern.clone().repeat(n_repeats),
        };
        total += solve(case.clone());
    }
    total
}

fn part1(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    call_solve(&data, 1)
}

fn part2(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    call_solve(&data, 5)
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
    const TEST_FILENAME: &str = "12.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 21)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 525152)
    }

    #[test]
    fn test_solve() {
        let file_contents = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let input = file_contents.strip_suffix('\n').unwrap();
        let true_values = vec![1, 4, 1, 1, 4, 10];
        for (test_case, true_value) in parse_input(input).iter().zip(true_values.iter()) {
            assert_eq!(*true_value, solve(test_case.clone()));
        }
    }

    #[test]
    fn test_solve_5_reps_additional_cases() {
        let test_case = Record {
            springs: vec![Condition::Operational, Condition::Damaged],
            pattern: vec![1],
        };
        assert_eq!(1, call_solve(&vec![test_case], 5));
    }

    #[test]
    fn test_solve_additional_cases() {
        assert_eq!(
            1,
            solve(Record {
                pattern: vec![1, 1],
                springs: parse_spring_string("#.#?.")
            })
        );
        assert_eq!(
            4,
            solve(Record {
                pattern: vec![1, 4, 2],
                springs: parse_spring_string(".???#?????.#?")
            })
        );
        assert_eq!(
            2,
            solve(Record {
                pattern: vec![1, 1, 1, 5],
                springs: parse_spring_string("??#??.???##?")
            })
        );
        assert_eq!(
            1,
            solve(Record {
                pattern: vec![1, 1],
                springs: parse_spring_string("???#.???#")
            })
        );
        assert_eq!(
            3,
            solve(Record {
                pattern: vec![3, 2, 4, 1, 2],
                springs: parse_spring_string("?##.???#?????#.#?.??")
            })
        );
    }
}
