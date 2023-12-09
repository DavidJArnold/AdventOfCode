const REAL_FILENAME: &str = "09.real.txt";

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn extrap(row: &[i64]) -> i64 {
    let mut r = row.to_owned();
    let mut y: i64 = *r.last().unwrap();
    loop {
        if r.len() == 1 {
            panic!("Diffs havent become constant");
        }
        let mut r2 = vec![];
        for idx in 0..(r.len() - 1) {
            r2.push(r[idx + 1] - r[idx]);
        }
        y += *r2.last().unwrap();
        if r2.iter().all(|x| x == &0) {
            break;
        }
        r = r2;
    }

    y
}

fn extrap_left(row: &[i64]) -> i64 {
    let mut r = row.to_owned();
    let mut y: i64 = *r.first().unwrap();
    loop {
        if r.len() == 1 {
            panic!("Diffs havent become constant");
        }
        let mut r2 = vec![];
        for idx in 0..(r.len() - 1) {
            r2.push(r[idx] - r[idx + 1]);
        }
        y += *r2.first().unwrap();
        if r2.iter().all(|x| x == &0) {
            break;
        }
        r = r2;
    }

    y
}
fn part1(filename: &str) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    let mut ans: i64 = 0;
    for row in data {
        ans += extrap(&row);
    }
    ans
}

fn part2(filename: &str) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    let mut ans: i64 = 0;
    for row in data {
        ans += extrap_left(&row);
    }
    ans
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
    const TEST_FILENAME: &str = "09.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 114)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 2)
    }

    #[test]
    fn test_extrap() {
        assert_eq!(extrap(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrap(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrap(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
