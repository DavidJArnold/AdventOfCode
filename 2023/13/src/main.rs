use num::complex::Complex;

const REAL_FILENAME: &str = "13.real.txt";

fn parse_input(input: &str) -> Vec<Vec<Complex<i32>>> {
    let mut patterns = vec![];
    for pattern in input.strip_suffix('\n').unwrap_or(input).split("\n\n") {
        let mut rocks = vec![];
        let mut x = 1;
        let mut y = 1;
        for c in pattern.chars() {
            match c {
                '#' => rocks.push(Complex { re: x, im: y }),
                '\n' => {
                    y += 1;
                    x = 0
                }
                _ => (),
            }
            x += 1;
        }
        patterns.push(rocks);
    }
    patterns
}

fn _print_pattern(pattern: &[Complex<i32>]) {
    let mut s: Vec<char> = vec![];
    for y in
        pattern.iter().map(|x| x.im).min().unwrap()..pattern.iter().map(|x| x.im).max().unwrap() + 1
    {
        for x in pattern.iter().map(|x| x.re).min().unwrap()
            ..pattern.iter().map(|x| x.re).max().unwrap() + 1
        {
            if pattern.contains(&Complex { re: x, im: y }) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s.iter().collect::<String>());
}

fn reflection(pattern: &[Complex<i32>], num_smudges: usize) -> Option<i32> {
    let max_row = pattern.iter().map(|x| x.im).max().unwrap();
    let min_row = pattern.iter().map(|x| x.im).min().unwrap();
    for w in min_row..max_row {
        // shift over by some amount
        let shifted = pattern
            .iter()
            .map(|x| Complex {
                re: x.re,
                im: 2 * x.im - (2 * w + 1),
            })
            .collect::<Vec<Complex<i32>>>();
        let max_row_shifted = shifted.iter().map(|x| x.im).max().unwrap();
        let min_row_shifted = shifted.iter().map(|x| x.im).min().unwrap();
        if shifted
            .clone()
            .iter()
            .filter(|x| x.im.abs() <= max_row_shifted.min(-min_row_shifted))
            .filter(|x| {
                !shifted.contains(&Complex {
                    re: x.re,
                    im: -x.im,
                })
            })
            .count()
            == num_smudges
        {
            return Some(w);
        }
    }
    None
}

fn part1(filename: &str, num_smudges: usize) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    let mut total = 0;
    for pattern in data {
        let h = reflection(&pattern, num_smudges);
        let flipped_pattern = &pattern
            .iter()
            .map(|x| Complex { re: x.im, im: x.re })
            .collect::<Vec<Complex<i32>>>();
        let v = reflection(flipped_pattern, num_smudges);
        total += 100 * h.unwrap_or(0) + v.unwrap_or(0);
    }

    total
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename, 0);
    println!("Part 1: {}", ans1);
    let ans2 = part1(filename, 1);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "13.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 405)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 400)
    }
}
