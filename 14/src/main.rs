use std::collections::HashMap;

fn read_input(contents: &str) -> HashMap<(i32, i32), char> {
    let lines = contents.strip_suffix("\n").unwrap_or(contents).split("\n");
    let mut cave: HashMap<(i32, i32), char> = HashMap::new();
    for line in lines {
        let instrs = line
            .split(" -> ")
            .map(|x| {
                x.split(",")
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        for idx in 0..instrs.len() - 1 {
            let p1 = &instrs[idx];
            let p2 = &instrs[idx + 1];
            if p1[0] == p2[0] {
                // vertical line
                for l in 0..p1[1].abs_diff(p2[1]) + 1 {
                    cave.insert((p1[0], p1[1].min(p2[1]) + l as i32), '#');
                }
            }
            if p1[1] == p2[1] {
                // horizontal line
                for l in 0..p1[0].abs_diff(p2[0]) + 1 {
                    cave.insert((p1[0].min(p2[0]) + l as i32, p1[1]), '#');
                }
            }
        }
    }
    return cave;
}

fn print_cave(cave: &HashMap<(i32, i32), char>) {
    let cave_copy = cave.clone();
    println!("{:?}", cave_copy);
    let pts = cave_copy.keys();
    println!("{:?}", pts);
    let cs = pts.clone().map(|x| x.0).collect::<Vec<_>>();
    println!("{:?}", cs);
    let x_min = pts.clone().map(|x| x.0).min().unwrap();
    let x_max = pts.clone().map(|x| x.0).max().unwrap();
    let y_min = pts.clone().map(|x| x.1).min().unwrap();
    let y_max = pts.map(|x| x.1).max().unwrap();
    let mut arr: Vec<Vec<char>> = vec![];
    for _ in 0..(y_max + 3 - y_min) {
        let mut col = vec![];
        for _ in 0..(x_max + 3 - x_min) {
            col.push('.');
        }
        arr.push(col);
    }
    for (idx, x) in (x_min - 1..x_max + 1).enumerate() {
        for (jdx, y) in (y_min - 1..y_max + 1).enumerate() {
            arr[jdx][idx] = *cave_copy.get(&(x, y as i32)).unwrap_or(&'.');
        }
    }
    for row in arr {
        println!("{:?}", row.iter().collect::<String>());
    }
}

fn move_sand(
    cave: &mut HashMap<(i32, i32), char>,
    cond: impl Fn((i32, i32)) -> bool,
    height: i32,
) -> usize {
    let mut sand: (i32, i32) = (0, 0);
    while cond(sand) {
        sand = (500, 0);
        while sand.1 <= height + 2 {
            if !cave.contains_key(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if !cave.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !cave.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                cave.insert(sand, 'o');
                break;
            }
        }
    }
    return cave.values().filter(|x| x == &&'o').count();
}

fn part1(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut cave = read_input(&contents);
    // print_cave(&cave);
    let height = cave.clone().keys().map(|x| x.1).max().unwrap().to_owned();
    let cond = |x: (i32, i32)| x.1 <= height;
    return move_sand(&mut cave, cond, height);
}

fn part2(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut cave = read_input(&contents);
    let height = cave.clone().keys().map(|x| x.1).max().unwrap().to_owned();
    let cond = |x| x != (500, 0);
    for idx in (-height - 2)..(height + 3) {
        cave.insert((500 + idx as i32, height + 2), '#');
    }

    return move_sand(&mut cave, cond, height);
}

fn main() {
    let filename = "14.real.txt";
    let ans1 = part1(&filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(&filename);
    println!("Part 2: {}", ans2);
}
