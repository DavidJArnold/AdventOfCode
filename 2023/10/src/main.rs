use std::collections::HashMap;

const REAL_FILENAME: &str = "10.real.txt";

fn parse_input(input: &str) -> HashMap<(i32, i32), char> {
    let mut diagram: HashMap<(i32, i32), char> = HashMap::new();
    for (jdx, line) in input.split('\n').enumerate() {
        for (idx, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            };
            diagram.insert((idx as i32, jdx as i32), c);
        }
    }
    diagram
}

fn find_dirs(diagram: &HashMap<(i32, i32), char>, position: &(i32, i32)) -> [(i32, i32); 2] {
    let mut dirs: [(i32, i32); 2] = [(0, 0); 2];
    let mut idx = 0;
    // to left of point, need - or F or L
    if ['-', 'F', 'L'].contains(diagram.get(&(position.0 - 1, position.1)).unwrap_or(&'x')) {
        dirs[idx] = (-1, 0);
        idx += 1;
    }
    // to right of point, need - or J or 7
    if ['-', 'J', '7'].contains(diagram.get(&(position.0 + 1, position.1)).unwrap_or(&'x')) {
        dirs[idx] = (1, 0);
        idx += 1;
    }
    // to below of point, need | or F or 7
    if ['|', 'J', 'L'].contains(diagram.get(&(position.0, position.1 + 1)).unwrap_or(&'x'))
        && idx <= 1
    {
        dirs[idx] = (0, 1);
        idx += 1;
    }
    // to above of point, need | or F or L
    if ['|', 'F', '7'].contains(diagram.get(&(position.0, position.1 - 1)).unwrap_or(&'x'))
        && idx <= 1
    {
        dirs[idx] = (0, -1);
        idx += 1;
    }
    if idx < 2 {
        panic!("Didn't find two starting directions");
    }

    dirs
}

fn follow_path(
    diagram: &HashMap<(i32, i32), char>,
    position: &(i32, i32),
    start_dirs: &[(i32, i32); 2],
) -> (usize, Vec<(i32, i32)>) {
    let mut p1 = *position;
    let mut p2 = *position;
    let mut d1 = start_dirs[0];
    let mut d2 = start_dirs[1];
    let mut idx: usize = 0;
    let mut path: Vec<(i32, i32)> = vec![];
    path.push(p1);

    let dirs = HashMap::from([
        ('-', ((-1, 0), (1, 0))),
        ('|', ((0, -1), (0, 1))),
        ('L', ((1, 0), (0, -1))),
        ('J', ((-1, 0), (0, -1))),
        ('7', ((-1, 0), (0, 1))),
        ('F', ((1, 0), (0, 1))),
    ]);
    loop {
        p1 = (p1.0 + d1.0, p1.1 + d1.1);
        p2 = (p2.0 + d2.0, p2.1 + d2.1);

        path.push(p1);
        idx += 1;
        if p1 == p2 {
            break;
        }
        path.push(p2);
        let new_d1 = dirs.get(diagram.get(&p1).unwrap()).unwrap();
        if new_d1.0 == (-d1.0, -d1.1) {
            d1 = new_d1.1;
        } else {
            d1 = new_d1.0
        }
        let new_d2 = dirs.get(diagram.get(&p2).unwrap()).unwrap();
        if new_d2.0 == (-d2.0, -d2.1) {
            d2 = new_d2.1;
        } else {
            d2 = new_d2.0
        }
    }
    (idx, path)
}

fn part1(filename: &str) -> usize {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    // first find start position
    let start_data = data.clone();
    let start = start_data
        .iter()
        .find_map(|(key, &x)| if x == 'S' { Some(key) } else { None })
        .unwrap();

    // check for starting directions
    let start_dirs = find_dirs(&data, start);

    // follow path around
    let (ans, _) = follow_path(&data, start, &start_dirs);
    ans
}

fn find_area(path: Vec<(i32, i32)>, diagram: &HashMap<(i32, i32), char>) -> u32 {
    let xmin = diagram.keys().map(|x| x.0).min().unwrap();
    let xmax = diagram.keys().map(|x| x.0).max().unwrap();
    let ymin = diagram.keys().map(|x| x.1).min().unwrap();
    let ymax = diagram.keys().map(|x| x.1).max().unwrap();
    let mut area: u32 = 0;
    for y in ymin..ymax + 1 {
        let mut count = 0;
        for x in xmin..xmax + 1 {
            if path.contains(&(x, y)) {
                if ['|', 'F', '7'].contains(diagram.get(&(x, y)).unwrap()) {
                    count += 1;
                }
            } else if count % 2 == 1 {
                area += 1;
            }
        }
    }
    area
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let mut data = parse_input(input);
    let start_data = data.clone();
    let start = start_data
        .iter()
        .find_map(|(key, &x)| if x == 'S' { Some(key) } else { None })
        .unwrap();
    let start_dirs = find_dirs(&data, start);
    let (_, path) = follow_path(&data, start, &start_dirs);

    let dirs = HashMap::from([
        ('-', ((-1, 0), (1, 0))),
        ('|', ((0, 1), (0, -1))),
        ('L', ((1, 0), (0, -1))),
        ('J', ((-1, 0), (0, -1))),
        ('7', ((-1, 0), (0, 1))),
        ('F', ((1, 0), (0, 1))),
    ]);

    let start_val = dirs
        .iter()
        .find_map(|(k, v)| {
            if [v.0, v.1] == start_dirs {
                Some(k)
            } else {
                None
            }
        })
        .unwrap();
    data.insert(*start, *start_val);
    find_area(path, &data)
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
    const TEST_FILENAME_1: &str = "10.test.1.txt";
    const TEST_FILENAME_2: &str = "10.test.2.txt";
    const TEST_FILENAME_3: &str = "10.test.3.txt";
    const TEST_FILENAME_4: &str = "10.test.4.txt";
    const TEST_FILENAME_5: &str = "10.test.5.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME_1);
        assert_eq!(ans1, 4)
    }

    #[test]
    fn test_part1_1() {
        let ans1 = part1(&TEST_FILENAME_2);
        assert_eq!(ans1, 8)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME_3);
        assert_eq!(ans2, 4)
    }

    #[test]
    fn test_part2_2() {
        let ans2 = part2(&TEST_FILENAME_4);
        assert_eq!(ans2, 10)
    }

    #[test]
    fn test_part2_3() {
        let ans2 = part2(&TEST_FILENAME_5);
        assert_eq!(ans2, 8)
    }
}
