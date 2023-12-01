use std::collections::HashMap;

const REAL_FILENAME: &str = "21.real.txt";

#[derive(Debug, Copy, Clone)]
enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
}

#[derive(Debug, Copy, Clone)]
struct Operation<'a> {
    left: &'a str,
    right: &'a str,
    op_type: OpType,
}

#[derive(Debug, Clone)]
struct Monkey<'a> {
    value: Option<f64>,
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
                "=" => OpType::Equals,
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
                    value: split_line[1].parse::<f64>().ok(),
                    operation: None,
                },
            );
        } else {
            panic!("What is this line? {line}");
        }
    }
    monkeys
}

fn do_op(op_type: OpType, left: f64, right: f64) -> f64 {
    match op_type {
        OpType::Minus => left - right,
        OpType::Plus => left + right,
        OpType::Multiply => left * right,
        OpType::Divide => left / right,
        OpType::Equals => right - left,
    }
}

fn solve(filename: &str, val: Option<f64>) -> f64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let file_text = file_contents.strip_suffix('\n').unwrap();
    let mut monkeys = parse_input(file_text);

    if let Some(x) = val {
        let root_monkey = monkeys.get_mut("root").unwrap();
        root_monkey.operation = Some(Operation {
            left: root_monkey.operation.unwrap().left,
            right: root_monkey.operation.unwrap().right,
            op_type: OpType::Equals,
        });

        let humn = monkeys.get_mut("humn").unwrap();
        humn.value = Some(x);
    };

    while monkeys.get("root").unwrap().value.is_none() {
        let monkeys_copy = monkeys.clone();
        let monkeys_without_values = monkeys_copy
            .iter()
            .filter(|x| x.1.value.is_none())
            .map(|x| x.0)
            .collect::<Vec<_>>();

        for monkey_name in monkeys_without_values {
            let left_val = monkeys
                .get(monkeys.get(monkey_name).unwrap().operation.unwrap().left)
                .unwrap()
                .value;
            let right_val = monkeys
                .get(monkeys.get(monkey_name).unwrap().operation.unwrap().right)
                .unwrap()
                .value;
            if let (Some(left), Some(right)) = (left_val, right_val) {
                let monkey = monkeys.get_mut(monkey_name).unwrap();
                monkey.value = Some(do_op(
                    monkey.operation.unwrap().op_type,
                    left,
                    right,
                ));
            }
        }
    }

    monkeys.get("root").unwrap().value.unwrap()
}

fn part2(filename: &str) -> f64 {
    let mut val = 10_000_000_000_000.0;
    // let mut val = 301;
    let mut step_size: i64 = val as i64;
    let mut step_direction = 1;
    let mut last_error_sign: f64 = 0.0;
    let mut last_error_mag: f64 = 0.0;
    loop {
        let error = solve(filename, Some(val));
        if error == 0.0 {
            return val;
        }
        if error.signum() != last_error_sign {
            step_direction = -step_direction;
            step_size = 1.max(step_size / 10);
        } else if error.abs() > last_error_mag {
            step_size = 1.max(step_size / 10);
        }
        last_error_sign = error.signum();
        last_error_mag = error.abs();
        val += step_size as f64 * step_direction as f64;
    }
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = solve(filename, None);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "21.test.txt";
    use crate::{part2, solve};

    #[test]
    fn test_part1() {
        let ans1 = solve(&TEST_FILENAME, None);
        assert_eq!(ans1, 152.0)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 301.0)
    }
}
