const REAL_FILENAME: &str = "11.real.txt";

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut galaxies: Vec<(i64, i64)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    galaxies
}

fn solve(data: &Vec<(i64, i64)>, gap_size: i64) -> i64 {
    let mut empty_rows: Vec<i64> = vec![];
    let x: Vec<i64> = data.iter().map(|x| x.0).collect();
    for a in 0..*x.iter().max().unwrap() {
        if !x.contains(&a) {
            empty_rows.push(a);
        }
    }

    let mut empty_cols: Vec<i64> = vec![];
    let y: Vec<i64> = data.iter().map(|x| x.1).collect();
    for a in 0..*y.iter().max().unwrap() {
        if !y.contains(&a) {
            empty_cols.push(a);
        }
    }

    let mut total_dist = 0;
    for idx in 0..data.len() {
        for jdx in idx + 1..data.len() {
            let p1 = data[idx];
            let p2 = data[jdx];
            let mut d = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
            for col in &empty_rows {
                if &p1.0.min(p2.0) < col && &p1.0.max(p2.0) > col {
                    d += gap_size;
                }
            }
            for row in &empty_cols {
                if &p1.1.min(p2.1) < row && &p1.1.max(p2.1) > row {
                    d += gap_size;
                }
            }
            total_dist += d;
        }
    }
    total_dist
}

fn part1(filename: &str) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    solve(&data, 1)
}

fn part2(filename: &str) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    solve(&data, 1_000_000 - 1)
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
    const TEST_FILENAME: &str = "11.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 374)
    }

    #[test]
    fn test_part2() {
        let file_contents = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let input = file_contents.strip_suffix('\n').unwrap();
        let data = parse_input(input);
        assert_eq!(solve(&data, 10 - 1), 1030);
        assert_eq!(solve(&data, 100 - 1), 8410);
    }
}
