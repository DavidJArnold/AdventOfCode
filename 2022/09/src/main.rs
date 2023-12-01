use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    direction: char,
    distance: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn get_input(filename: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for ele in std::fs::read_to_string(filename)
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
    {
        let ele_it = ele.split(" ").collect::<Vec<_>>();
        let instr = Instruction {
            direction: ele_it[0].parse::<char>().unwrap(),
            distance: ele_it[1].parse::<u32>().unwrap(),
        };
        instructions.push(instr)
    }
    return instructions;
}

fn move_tail(rope: &mut [Position]) {
    let delta = Position {
        x: rope[0].x - rope[1].x,
        y: rope[0].y - rope[1].y,
    };
    let dist = delta.x.pow(2) + delta.y.pow(2);
    if dist == 4 {
        rope[1].x += delta.x / 2;
        rope[1].y += delta.y / 2;
    } else if dist > 4 {
        rope[1].x += delta.x.signum();
        rope[1].y += delta.y.signum();
    }
}

fn solve(input: &Vec<Instruction>, num_knots: usize) -> usize {
    let mut visited_set = HashSet::new();
    let mut rope = vec![];
    for _ in 0..num_knots {
        rope.push(Position { x: 0, y: 0 });
    }
    for instr in input {
        for _ in 0..instr.distance {
            match instr.direction {
                'L' => rope[0].x -= 1,
                'R' => rope[0].x += 1,
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                _ => panic!("Non-expected direction found"),
            }
            for idx in 1..num_knots {
                move_tail(&mut rope[idx - 1..idx + 1]);
            }
            visited_set.insert(rope[num_knots - 1]);
        }
    }
    return visited_set.len();
}

fn main() {
    let input = get_input("09.real.txt");
    let ans1 = solve(&input, 2);
    println!("Part 1: {}", ans1);
    let ans2 = solve(&input, 10);
    println!("Part 2: {}", ans2);
}
