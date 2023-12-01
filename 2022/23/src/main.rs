use std::collections::{HashMap, HashSet};

const REAL_FILENAME: &str = "23.real.txt";

fn parse_input(input: &str) -> HashMap<usize, (i32, i32)> {
    let mut elves: HashMap<usize, (i32, i32)> = HashMap::new();
    let mut idx: usize = 0;
    let height = input.clone().split('\n').count();
    for (j, row) in input.split('\n').enumerate() {
        for (i, spot) in row.chars().enumerate() {
            if spot == '#' {
                elves.insert(idx, (i as i32, (height - j - 1) as i32));
                idx += 1;
            }
        }
    }
    elves
}

fn move_elves(
    elves: HashMap<usize, (i32, i32)>,
    round_num: usize,
) -> (HashMap<usize, (i32, i32)>, bool) {
    let mut proposed: HashMap<usize, (i32, i32)> = HashMap::new();
    let dirs = [
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (-1, 0), (-1, -1)],
        [(1, -1), (1, 0), (1, 1)],
    ];
    let around = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let elf_coords: HashSet<(i32, i32)> =
        elves
            .clone()
            .values()
            .fold(HashSet::new(), |mut accum, item| {
                accum.insert(*item);
                accum
            });

    for (idx, elf) in elves.clone().iter() {
        let _ = proposed.insert(*idx, *elf);
        if around
            .iter()
            .fold(HashSet::new(), |mut accum, item| {
                accum.insert((item.0 + elf.0, item.1 + elf.1));
                accum
            })
            .is_disjoint(&elf_coords)
        {
            continue;
        }
        for id_ in 0..4 {
            let dir_idx = (id_ + round_num) % 4;
            let test_pos = dirs[dir_idx]
                .iter()
                .fold(HashSet::new(), |mut accum, item| {
                    accum.insert((item.0 + elf.0, item.1 + elf.1));
                    accum
                });
            if test_pos.is_disjoint(&elf_coords) {
                proposed.insert(
                    *idx,
                    (elf.0 + dirs[dir_idx][1].0, elf.1 + dirs[dir_idx][1].1),
                );
                break;
            }
        }
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut seen_mult: HashSet<(i32, i32)> = HashSet::new();
    for coord in proposed.values() {
        if seen.contains(coord) {
            seen_mult.insert(*coord);
        } else {
            seen.insert(*coord);
        }
    }

    let seen_multiple = proposed
        .keys()
        .filter(|x| seen_mult.contains(proposed.get(x).unwrap()))
        .collect::<Vec<_>>();
    let not_seen = proposed
        .keys()
        .filter(|x| !seen_multiple.contains(x))
        .collect::<Vec<_>>();

    let mut new: HashMap<usize, (i32, i32)> = HashMap::new();

    for idx in seen_multiple {
        new.insert(*idx, *elves.get(idx).unwrap());
    }
    for idx in not_seen {
        new.insert(*idx, *proposed.get(idx).unwrap());
    }

    let changed = elves != new;
    (new, changed)
}

fn print_elves(elves: &HashMap<usize, (i32, i32)>) {
    let min_x = elves.values().map(|x| x.0).min().unwrap();
    let max_x = elves.values().map(|x| x.0).max().unwrap();
    let min_y = elves.values().map(|x| x.1).min().unwrap();
    let max_y = elves.values().map(|x| x.1).max().unwrap();
    let mut output = String::new();
    for jdx in min_y..max_y + 2 {
        for idx in min_x..max_x + 1 {
            if elves
                .values()
                .collect::<Vec<_>>()
                .contains(&&(idx, max_y - jdx))
            {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }
    println!("{output}");
}

fn part1(filename: &str) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.strip_suffix('\n').unwrap();
    let mut elves = parse_input(lines);

    for round_num in 0..10 {
        (elves, _) = move_elves(elves, round_num);
    }

    let width =
        elves.values().map(|x| x.0).max().unwrap() - elves.values().map(|x| x.0).min().unwrap() + 1;
    let height =
        elves.values().map(|x| x.1).max().unwrap() - elves.values().map(|x| x.1).min().unwrap() + 1;

    width * height - elves.len() as i32
}

fn part2(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.strip_suffix('\n').unwrap();
    let mut elves = parse_input(lines);
    let mut changed = true;
    let mut round_num = 0;
    while changed {
        (elves, changed) = move_elves(elves, round_num);
        round_num += 1;
    }
    round_num
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
    const TEST_FILENAME: &str = "23.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 110);
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 20)
    }
}
