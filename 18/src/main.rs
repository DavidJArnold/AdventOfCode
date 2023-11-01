use std::collections::HashSet;

const REAL_FILENAME: &str = "18.real.txt";

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn count_exposed(coords: Vec<Vec<i32>>) -> usize {
    let mut matches = 0;
    for cube in &coords {
        for other_cube in &coords {
            let line: usize = (0_usize..3_usize)
                .filter(|x| cube[*x] == other_cube[*x])
                .count();
            let diff: i32 = (0_usize..3_usize)
                .map(|x| (cube[x] - other_cube[x]).abs())
                .sum();
            if line == 2 && diff == 1 {
                matches += 1;
            }
        }
    }

    6 * coords.len() - matches
}

fn find_neighbours(
    drops: &HashSet<(i32, i32, i32)>,
    cube_set: &mut HashSet<(i32, i32, i32)>,
    min: i32,
    max: i32,
) {
    // &mut HashSet<(i32, i32, i32)> {
    for cube in cube_set.clone() {
        let mut c0 = cube;
        c0.0 += 1;
        let mut c1 = cube;
        c1.0 -= 1;
        let mut c2 = cube;
        c2.1 += 1;
        let mut c3 = cube;
        c3.1 -= 1;
        let mut c4 = cube;
        c4.2 += 1;
        let mut c5 = cube;
        c5.2 -= 1;
        let neighbours = [c0, c1, c2, c3, c4, c5];
        for neighbour in neighbours {
            if !drops.contains(&neighbour)
                && neighbour.0.max(neighbour.1).max(neighbour.2) <= max + 1
                && neighbour.0.min(neighbour.1).min(neighbour.2) >= min - 1
            {
                cube_set.insert(neighbour);
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let coords = parse_input(&input);
    count_exposed(coords)
}

fn part2(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    let coords = parse_input(&input);
    let mut drops: HashSet<(i32, i32, i32)> = HashSet::new();
    for coord in &coords {
        drops.insert((coord[0], coord[1], coord[2]));
    }

    // bounding limits
    let min = coords
        .iter()
        .map(|x| x.iter().min().unwrap())
        .min()
        .unwrap();
    let max = coords
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap();

    let mut cube_set: HashSet<(i32, i32, i32)> = HashSet::new();
    for x in (min - 1)..(max + 2) {
        for y in (min - 1)..(max + 2) {
            cube_set.insert((x, y, min - 1));
            cube_set.insert((x, y, max + 1));
        }
    }
    for y in (min - 1)..(max + 2) {
        for z in (min - 1)..(max + 2) {
            cube_set.insert((min - 1, y, z));
            cube_set.insert((max + 1, y, z));
        }
    }
    for z in (min - 1)..(max + 2) {
        for x in (min - 1)..(max + 2) {
            cube_set.insert((x, min - 1, z));
            cube_set.insert((x, max + 1, z));
        }
    }

    let mut l = cube_set.len();
    loop {
        find_neighbours(&drops, &mut cube_set, *min, *max);
        if cube_set.len() <= l {
            break;
        }
        l = cube_set.len();
    }

    let mut inside_cubes: Vec<Vec<i32>> = vec![];
    for x in (min - 1)..(max + 2) {
        for y in (min - 1)..(max + 2) {
            for z in (min - 1)..(max + 2) {
                if !drops.contains(&(x, y, z)) && !cube_set.contains(&(x, y, z)) {
                    inside_cubes.push(vec![x, y, z]);
                }
            }
        }
    }
    
    count_exposed(coords) - count_exposed(inside_cubes)
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
    const TEST_FILENAME: &str = "18.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 64)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 58)
    }
}
