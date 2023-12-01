use std::collections::{HashMap, HashSet};

const REAL_FILENAME: &str = "24.real.txt";

type Position = (i32, i32);
type Field = Vec<(Position, char)>;

fn parse_input(input: &str) -> Field {
    let mut field = vec![];
    for (j, row) in input.split('\n').enumerate() {
        for (i, entry) in row.chars().enumerate() {
            if entry != '.' {
                field.push(((i as i32, j as i32), entry));
            }
        }
    }
    field
}

fn print_field(field: &mut Field, pos: Option<HashMap<Position, i32>>) {
    let mut field_map: HashMap<Position, char> = HashMap::new();
    for el in field.iter() {
        field_map.insert(el.0, el.1);
    }
    println!("{:?}", field_map);
    if let Some(posi) = pos {
        for p in posi {
            field_map.insert(p.0, 'E');
        }
    }
    let min_x = field_map.keys().map(|x| x.0).min().unwrap();
    let max_x = field_map.keys().map(|x| x.0).max().unwrap();
    let min_y = field_map.keys().map(|x| x.1).min().unwrap();
    let max_y = field_map.keys().map(|x| x.1).max().unwrap();
    let mut print_str = String::new();
    for j in min_y..max_y + 1 {
        for i in min_x..max_x + 1 {
            print_str += &field_map.get(&(i, j)).unwrap_or(&'.').to_string();
        }
        print_str += "\n";
    }
    println!("{print_str}");
}

fn move_wind(field: &mut Field, width: i32, height: i32) -> &mut Field {
    let mut walls: HashSet<Position> = HashSet::new();
    for x in field.clone().iter() {
        if x.1 == '#' {
            walls.insert(x.0);
        }
    }

    let mut dirs: HashMap<char, Position> = HashMap::new();
    dirs.insert('>', (1, 0));
    dirs.insert('<', (-1, 0));
    dirs.insert('^', (0, -1));
    dirs.insert('v', (0, 1));
    for (idx, elem) in field.clone().iter().enumerate() {
        if dirs.contains_key(&elem.1) {
            let mut new_coord = (
                elem.0 .0 + dirs.get(&elem.1).unwrap().0,
                elem.0 .1 + dirs.get(&elem.1).unwrap().1,
            );

            let size_: i32;
            if walls.contains(&new_coord) {
                if elem.1 == '>' || elem.1 == '<' {
                    size_ = width;
                } else if elem.1 == '^' || elem.1 == 'v' {
                    size_ = height;
                } else {
                    panic!()
                }
                new_coord = (
                    new_coord.0 - size_ * dirs.get(&elem.1).unwrap().0,
                    new_coord.1 - size_ * dirs.get(&elem.1).unwrap().1,
                );
            }
            if let Some(field_elem) = field.get_mut(idx) {
                *field_elem = (new_coord, elem.1);
            }
        }
    }

    field
}

fn is_valid(candidate_pos: Position, blocked: &HashSet<Position>, width: i32, height: i32) -> bool {
    if candidate_pos.0 < 0 {
        false
    } else if candidate_pos.0 > width {
        false
    } else if candidate_pos.1 < 0 {
        false
    } else if candidate_pos.1 > height {
        false
    } else {
        !blocked.contains(&candidate_pos)
    }
}

fn traverse(
    mut field: &mut Field,
    start: Position,
    end: Position,
    width: i32,
    height: i32,
) -> (i32, Field) {
    let mut pos: HashMap<Position, i32> = HashMap::new();
    pos.insert(start, 0);
    let mut idx: i32 = 1;
    loop {
        field = move_wind(field, width - 1, height - 1);

        let mut blocked: HashSet<Position> = HashSet::new();
        for p in field.iter() {
            blocked.insert(p.0);
        }

        let mut new_pos: HashMap<Position, i32> = HashMap::new();
        let dirs: Vec<Position> = vec![(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];

        for p in pos {
            for dir in &dirs {
                let candidate_pos = (p.0 .0 + dir.0, p.0 .1 + dir.1);
                if is_valid(candidate_pos, &blocked, width, height) {
                    new_pos.insert(candidate_pos, idx);
                }
            }
        }
        pos = new_pos.clone();

        if pos.contains_key(&end) {
            return (idx, field.clone());
        }

        idx += 1;
    }
}

fn part1(filename: &str) -> i32 {
    let file_content = std::fs::read_to_string(filename).unwrap();
    let input = file_content.strip_suffix('\n').unwrap();
    let mut field = parse_input(input);

    let width = field.clone().iter().map(|x| x.0 .0).max().unwrap();
    let height = field.clone().iter().map(|x| x.0 .1).max().unwrap();

    let (num, _) = traverse(&mut field, (1, 0), (width - 1, height), width, height);
    num
}

fn part2(filename: &str) -> i32 {
    let file_content = std::fs::read_to_string(filename).unwrap();
    let input = file_content.strip_suffix('\n').unwrap();
    let mut field = parse_input(input);

    let width = field.clone().iter().map(|x| x.0 .0).max().unwrap();
    let height = field.clone().iter().map(|x| x.0 .1).max().unwrap();

    let start = (1, 0);
    let end = (width - 1, height);

    let (num1, mut field) = traverse(&mut field, start, end, width, height);
    let (num2, mut field) = traverse(&mut field, end, start, width, height);
    let (num3, _) = traverse(&mut field, start, end, width, height);
    num1 + num2 + num3
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
    const TEST_FILENAME: &str = "24.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 18)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 54)
    }
}
