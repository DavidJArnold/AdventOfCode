use std::collections::HashMap;

const REAL_FILENAME: &str = "01.real.txt";

fn calculate_calibration(lines: Vec<String>) -> u32 {
    let mut calibration = 0;
    for line in lines {
        let mut digits: Vec<u32> = vec![];
        for dig in line.chars() {
            if ['1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&dig) {
                digits.push(dig.to_digit(10).unwrap())
            }
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap_or(first);
        calibration += first * 10 + last
    }
    calibration
}

fn parse_line(line: &str) -> String {
    let translate: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut newline: String = "".to_string();
    for idx in 0..line.len() {
        if translate
            .values()
            .collect::<Vec<_>>()
            .contains(&&line.chars().nth(idx).unwrap().to_digit(10).unwrap_or(0))
        {
            let new_str = &line.chars().nth(idx).unwrap();
            newline = format!("{newline}{}", new_str);
            continue;
        }
        for jdx in 3..6 {
            if idx + jdx <= line.len()
                && translate
                    .keys()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .contains(&line.chars().skip(idx).take(jdx).collect::<String>())
            {
                let digit_string = &line.chars().clone().skip(idx).take(jdx).collect::<String>();
                let new_str = &translate.get(digit_string.as_str()).unwrap();
                newline = format!("{newline}{}", new_str);
            };
        }
    }
    newline.to_string()
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    calculate_calibration(input.split('\n').map(|x| x.to_string()).collect::<Vec<_>>())
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let parsed_input = input.split('\n').map(parse_line).collect::<Vec<_>>();
    calculate_calibration(parsed_input)
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
    const TEST_FILENAME: &str = "01.test.txt";
    const TEST_FILENAME2: &str = "01.test2.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 142)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME2);
        assert_eq!(ans2, 281)
    }

    #[test]
    fn test_input_parsing() {
        assert_eq!(parse_line("asdoneasd"), "1");
        assert_eq!(parse_line("twoasdoneasd"), "21");
        assert_eq!(parse_line("twoneasdone4asd"), "2114");
    }
}
