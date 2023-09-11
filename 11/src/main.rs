#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    test: u64,
    operation: Operation,
    num: u64,
    if_false: u64,
    if_true: u64,
    worry_count: u64,
}

fn parse_input(text: &str) -> Vec<Monkey> {
    let contents = text
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .collect::<Vec<_>>();
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey in &contents {
        let monkey_lines = monkey.split("\n").collect::<Vec<_>>();
        let mut op = monkey_lines[2].split(" ").filter(|s| !s.is_empty()).skip(4);
        let op_type;
        let op_symbol = op.next().unwrap();
        let is_arg_old = op.next().unwrap();
        if is_arg_old == "old" {
            op_type = Operation::Square;
        } else if op_symbol == "+" {
            op_type = Operation::Add;
        } else if op_symbol == "*" {
            op_type = Operation::Multiply;
        } else {
            panic!("Couldn't parse op type");
        };

        monkeys.push(Monkey {
            items: monkey_lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
            test: monkey_lines[3]
                .split(" ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            operation: op_type,
            num: monkey_lines[2]
                .split(" ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap_or(0),
            if_false: monkey_lines[5]
                .split(" ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            if_true: monkey_lines[4]
                .split(" ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            worry_count: 0,
        });
    }
    return monkeys;
}

fn part1(monkeys: &mut Vec<Monkey>) -> u64 {
    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            for val in monkeys[idx].items.clone() {
                let new_val = match monkeys[idx].operation {
                    Operation::Add => val + monkeys[idx].num,
                    Operation::Multiply => val * monkeys[idx].num,
                    Operation::Square => val * val,
                };
                let new_monkey: u64 = if (new_val / 3) % monkeys[idx].test == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                };
                monkeys[idx].worry_count += 1;
                monkeys[new_monkey as usize].items.push(new_val / 3);
                // println!("Monkey {} throws item with worry level {} to monkey {}", idx, new_val / 3, new_monkey);
            }
            monkeys[idx].items.clear();
        }
        // println!("After round {}", round+1);
        // for idx in 0..monkeys.len() {
        //     println!("Monkey {idx}: {:?}", monkeys[idx].items);
        // }
    }
    let mut worries = monkeys.iter().map(|x| x.worry_count).collect::<Vec<_>>();
    worries.sort();
    return worries.iter().rev().take(2).product()
}

fn part2(monkeys: &mut Vec<Monkey>) -> u64 {
    let denom: u64 = monkeys.clone().iter().map(|x| x.test).product();
    for _round in 0..10_000 {
        for idx in 0..monkeys.len() {
            for val in monkeys[idx].items.clone() {
                let full_new_val = match monkeys[idx].operation {
                    Operation::Add => val + monkeys[idx].num,
                    Operation::Multiply => val * monkeys[idx].num,
                    Operation::Square => val * val,
                };
                let new_val = full_new_val % denom;
                let new_monkey: u64 = if new_val % monkeys[idx].test == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                };
                monkeys[idx].worry_count += 1;
                monkeys[new_monkey as usize].items.push(new_val);
                // println!("Monkey {} throws item with worry level {} to monkey {}", idx, new_val / 3, new_monkey);
            }
            monkeys[idx].items.clear();
        }
        // println!("After round {}", round+1);
        // for idx in 0..monkeys.len() {
        //     println!("Monkey {idx}: {:?}", monkeys[idx].items);
        // }
    }
    let mut worries = monkeys.iter().map(|x| x.worry_count).collect::<Vec<_>>();
    worries.sort();
    return worries.iter().rev().take(2).product()
}

fn main() {
    let file_content = std::fs::read_to_string("11.real.txt").unwrap();
    let mut input = parse_input(&file_content);
    let ans1 = part1(&mut input.clone());
    println!("Part 1: {}", ans1);
    let ans2 = part2(&mut input);
    println!("Part 2: {}", ans2);
}
