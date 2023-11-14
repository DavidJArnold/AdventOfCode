use std::collections::HashMap;

const REAL_FILENAME: &str = "21.real.txt";

#[derive(Debug, Copy, Clone)]
enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Copy, Clone)]
struct Operation<'a> {
    left: &'a str,
    right: &'a str,
    op_type: OpType,
}

#[derive(Debug, Clone)]
struct Monkey<'a> {
    value: Option<i64>,
    operation: Option<Operation<'a>>,
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();

    for line in input.split('\n') {
        let split_line: Vec<&str> = line.split(' ').collect();
        if split_line.len() == 4 {
            let op_enum = match split_line[2] {
                "-" => OpType::Minus,
                "+" => OpType::Plus,
                "*" => OpType::Multiply,
                "/" => OpType::Divide,
                x => panic!("Unknown operation {x}"),
            };
            let op = Operation {
                left: split_line[1],
                right: split_line[3],
                op_type: op_enum,
            };
            monkeys.insert(
                split_line[0].strip_suffix(':').unwrap(),
                Monkey {
                    value: None,
                    operation: Some(op),
                },
            );
        } else if split_line.len() == 2 {
            monkeys.insert(
                split_line[0].strip_suffix(':').unwrap(),
                Monkey {
                    value: split_line[1].parse::<i64>().ok(),
                    operation: None,
                },
            );
        } else {
            panic!("What is this line? {line}");
        }
    }
    monkeys
}

fn do_op(op_type: OpType, left: i64, right: i64) -> i64 {
    match op_type {
        OpType::Minus => left - right,
        OpType::Plus => left + right,
        OpType::Multiply => left * right,
        OpType::Divide => left / right,
    }
}

fn part1(filename: &str) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let file_text = file_contents.strip_suffix('\n').unwrap();
    let mut monkeys = parse_input(file_text);

    while monkeys.get("root").unwrap().value.is_none() {
        let monkeys_copy = monkeys.clone();
        let monkeys_without_values = monkeys_copy
            .iter()
            .filter(|x| x.1.value.is_none())
            .map(|x| x.0).collect::<Vec<_>>();

        for monkey_name in monkeys_without_values {
            let left_val = monkeys
                .get(monkeys.get(monkey_name).unwrap().operation.unwrap().left)
                .unwrap()
                .value;
            let right_val = monkeys
                .get(monkeys.get(monkey_name).unwrap().operation.unwrap().right)
                .unwrap()
                .value;
            if left_val.is_some() && right_val.is_some() {
                let monkey = monkeys.get_mut(monkey_name).unwrap();
                monkey.value = Some(do_op(
                    monkey.operation.unwrap().op_type,
                    left_val.unwrap(),
                    right_val.unwrap(),
                ));
            }
        }
    }

    return monkeys.get("root").unwrap().value.unwrap();
}

fn part2(filename: &str) -> i64 {
    return 0;
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
    const TEST_FILENAME: &str = "21.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 152)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 301)
    }
}
