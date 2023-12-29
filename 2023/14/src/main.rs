use std::collections::HashSet;

const REAL_FILENAME: &str = "14.real.txt";

type Coords= HashSet<(i32, i32)>;

fn parse_input(input: &str) -> (Coords, Coords) {
    let mut roller_data: Vec<(i32, i32)> = vec![];
    let mut rock_data: Vec<(i32, i32)> = vec![];
    let mut last_row = 0;
    for (idx, row) in input.split('\n').enumerate() {
        for (jdx, chr) in row.chars().enumerate() {
            match chr {
                '#' => rock_data.push((jdx as i32, idx as i32)),
                'O' => roller_data.push((jdx as i32, idx as i32)),
                _ => (),
            }
        }
        last_row = idx;
    }
    let roller_set = roller_data.iter().map(|x| (x.0, last_row as i32 - x.1));
    let rollers: Coords = HashSet::from_iter(roller_set);
    let rock_set = rock_data.iter().map(|x| (x.0, last_row as i32 - x.1));
    let rocks: Coords = HashSet::from_iter(rock_set);
    (rollers, rocks)
}

fn print_load(rollers: &Coords, rocks: &Coords) {
    let mut out: Vec<char> = vec![];
    let height = rollers
        .iter()
        .map(|x| x.1)
        .max()
        .unwrap()
        .max(rocks.iter().map(|x| x.1).max().unwrap());
    for idx in 0..height + 1 {
        for jdx in 0..rollers
            .iter()
            .map(|x| x.0)
            .max()
            .unwrap()
            .max(rocks.iter().map(|x| x.0).max().unwrap())
            + 1
        {
            if rollers.contains(&(jdx, height - idx)) {
                out.push('O');
            } else if rocks.contains(&(jdx, height - idx)) {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        println!("{} {}", out.iter().collect::<String>(), height - idx);
        out = vec![];
    }
}

fn roll(rollers: &Coords, rocks: &Coords) -> Coords {
    let max_height = rollers
        .iter()
        .map(|x| x.1)
        .max()
        .unwrap()
        .max(rocks.iter().map(|x| x.1).max().unwrap());
    let mut temp_rollers = rollers.clone();
    let mut moved = true;
    while moved {
        moved = false;
        let new_rollers = temp_rollers.clone();
        temp_rollers = HashSet::new();
        for roller in &new_rollers {
            if !new_rollers.contains(&(roller.0, roller.1 + 1))
                && !rocks.contains(&(roller.0, roller.1 + 1))
                && roller.1 < max_height
            {
                temp_rollers.insert((roller.0, roller.1 + 1));
                moved = true;
            } else {
                temp_rollers.insert(*roller);
            }
        }
    }
    temp_rollers
}

fn roll_dir(
    rollers: &Coords,
    rocks: &Coords,
    dir: (i32, i32),
) -> Coords {
    let max_height = rollers
        .iter()
        .map(|x| x.1)
        .max()
        .unwrap()
        .max(rocks.iter().map(|x| x.1).max().unwrap());
    let max_width = rollers
        .iter()
        .map(|x| x.0)
        .max()
        .unwrap()
        .max(rocks.iter().map(|x| x.0).max().unwrap());
    let mut temp_rollers = rollers.clone();
    let mut moved = true;
    while moved {
        moved = false;
        let new_rollers = temp_rollers.clone();
        temp_rollers = HashSet::new();
        for roller in &new_rollers {
            if !new_rollers.contains(&(roller.0 + dir.0, roller.1 + dir.1))
                && !rocks.contains(&(roller.0 + dir.0, roller.1 + dir.1))
                && roller.1 + dir.1 <= max_height
                && roller.0 + dir.0 <= max_width
                && roller.1 + dir.1 >= 0
                && roller.0 + dir.0 >= 0
            {
                temp_rollers.insert((roller.0 + dir.0, roller.1 + dir.1));
                moved = true;
            } else {
                temp_rollers.insert(*roller);
            }
        }
    }
    temp_rollers
}

fn part1(filename: &str) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (rollers, rocks) = parse_input(input);

    let rolled_rollers = roll(&rollers, &rocks);
    rolled_rollers.iter().map(|x| x.1 + 1).sum::<i32>()
}

fn brent(rollers: Coords, rocks: Coords) -> (i32, i32) {
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = rollers.clone();
    let mut hare = four_roll_set(tortoise.clone(), &rocks);
    while tortoise != hare {
        if power == lam {
            tortoise = hare.clone();
            power *= 2;
            lam = 0;
        }
        hare = four_roll_set(hare, &rocks);
        lam += 1;
    }

    let mut tortoise = rollers.clone();
    let mut hare = rollers;
    for _ in 0..lam {
        hare = four_roll_set(hare, &rocks);
    }

    let mut mu = 0;
    while tortoise != hare {
        tortoise = four_roll_set(tortoise, &rocks);
        hare = four_roll_set(hare, &rocks);
        mu += 1;
    }

    (lam, mu)
}

fn four_roll_set(
    mut rolled_rollers: Coords,
    rocks: &Coords,
) -> Coords {
    rolled_rollers = roll_dir(&rolled_rollers, rocks, (0, 1));
    rolled_rollers = roll_dir(&rolled_rollers, rocks, (-1, 0));
    rolled_rollers = roll_dir(&rolled_rollers, rocks, (0, -1));
    roll_dir(&rolled_rollers, rocks, (1, 0))
}

fn part2(filename: &str) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (rollers, rocks) = parse_input(input);
    let mut rolled_rollers = rollers.clone();

    // cycle detection
    let (lam, mu) = brent(rollers, rocks.clone());
    // println!("Loop length {lam} starting after {mu} iterations");
    let n = 1000000000;
    let unloop = mu + ((n - mu) % lam);

    for _ in 0..unloop {
        rolled_rollers = four_roll_set(rolled_rollers, &rocks);
    }
    rolled_rollers.iter().map(|x| x.1 + 1).sum::<i32>()
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
    const TEST_FILENAME: &str = "14.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 136)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 64)
    }

    #[test]
    fn test_roll_dir() {
        let file_contents = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let input = file_contents.strip_suffix('\n').unwrap();
        let (rollers, rocks) = parse_input(input);

        print_load(&rollers, &rocks);
        let rolled_dir = roll_dir(&rollers, &rocks, (0, 1));
        let rolled = roll(&rollers, &rocks);
        println!("----------");
        print_load(&rolled_dir, &rocks);
        println!("----------");
        print_load(&rolled, &rocks);
        assert_eq!(rolled_dir, rolled);
    }
}
