use std::fs;

fn read_input(filename: &str) -> (Vec<Vec<i32>>, Vec<Vec<char>>) {
    let input = fs::read_to_string(filename).expect("Couldn't read input");
    let lines = input.split("\n").to_owned();

    // Find end of tower definition
    let mut s: usize = 0;
    for (id, ele) in lines.clone().enumerate() {
        if ele.chars().nth(1) == Some('1') {
            s = id;
            break;
        }
    }

    // get tower definitions, first parse input
    let input_lines = input.split("\n").to_owned();
    let towers = input_lines
        .take(s)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<String>().unwrap())
        .rev()
        .collect::<Vec<_>>();

    // then build empty vecs
    let num_towers = (towers[0].len() + 1) / 4;
    let mut tower_list: Vec<Vec<char>> = vec![];
    for _ in 0..num_towers {
        tower_list.push(vec![]);
    }

    // fill out the tower values row-by-row
    for el in towers.iter() {
        for j in 0..num_towers {
            let chr = el.chars().nth(4 * j + 1).unwrap();
            if chr != ' ' {
                tower_list[j].push(chr);
            };
        }
    }

    // then get instructions
    let input_length = lines.collect::<Vec<_>>();
    let instructions = input_length[s + 2..].to_owned();
    let instr = instructions
        .iter()
        .filter(|x| x.len() != 0)
        .map(|x| {
            x.split(" ")
                .enumerate()
                .filter(|&(i, _)| i == 1 || i == 3 || i == 5)
                .map(|(_, e)| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    return (instr, tower_list);
}

fn solve(
    filename: &str,
    move_fn: &dyn Fn(usize, Vec<char>, Vec<char>) -> (Vec<char>, Vec<char>),
) -> String {
    let (instructions, towers) = read_input(filename);
    let mut towers = towers.clone();
    let instrs = instructions.clone();
    for inst in instrs.iter() {
        let source = (inst[1] - 1) as usize;
        let target = (inst[2] - 1) as usize;
        let number = inst[0] as usize;
        let source_tower = towers.iter().nth(source).unwrap().to_owned();
        let target_tower = towers.iter().nth(target).unwrap().to_owned();
        let (a, b) = move_fn(number, source_tower, target_tower);
        towers[source] = a;
        towers[target] = b;
    }
    let final_result = towers
        .iter()
        .map(|x| x.last().unwrap_or(&' '))
        .collect::<String>();
    return final_result;
}

fn move_p1(
    number: usize,
    mut source_tower: Vec<char>,
    mut target_tower: Vec<char>,
) -> (Vec<char>, Vec<char>) {
    for _ in 0..number {
        let swap = source_tower.pop().unwrap();
        target_tower.push(swap)
    }
    return (source_tower, target_tower);
}

fn move_p2(
    number: usize,
    mut source_tower: Vec<char>,
    mut target_tower: Vec<char>,
) -> (Vec<char>, Vec<char>) {
    let source_tower_length = source_tower.len() - number;
    let swap = source_tower.split_off(source_tower_length);
    target_tower.extend(swap.iter());
    return (source_tower, target_tower);
}

fn main() {
    let filename = "05.real.txt";

    let p1 = solve(filename, &move_p1);
    let p2 = solve(filename, &move_p2);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
