const REAL_FILENAME: &str = "03.real.txt";

struct Number {
    value: u32,
    cols: Vec<usize>,
    row: usize,
}

struct Symbol {
    position: (usize, usize),
    symbol: char,
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    for (idx, line) in input.split('\n').enumerate() {
        let mut next_num: Vec<char> = vec![];
        let mut next_pos_col: Vec<usize> = vec![];

        for (jdx, chr) in line.chars().enumerate() {
            if chr.is_ascii_digit() {
                next_num.push(chr);
                next_pos_col.push(jdx);
            } else {
                if !next_num.is_empty() {
                    let number: u32 = next_num
                        .iter()
                        .fold(0, |acc, &digit| acc * 10 + digit.to_digit(10).unwrap());
                    numbers.push(Number {
                        value: number,
                        cols: next_pos_col,
                        row: idx,
                    });
                    next_num = vec![];
                    next_pos_col = vec![];
                }
                if chr != '.' {
                    symbols.push(Symbol {
                        position: (jdx, idx),
                        symbol: chr,
                    });
                }
            }
        }
        if !next_num.is_empty() {
            let number: u32 = next_num
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit.to_digit(10).unwrap());
            numbers.push(Number {
                value: number,
                cols: next_pos_col,
                row: idx,
            });
        }
    }

    (numbers, symbols)
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap_or(&file_contents);
    let (numbers, symbols) = parse_input(input);
    let sym_pos = symbols.iter().map(|x| x.position).collect::<Vec<_>>();

    let mut result: u32 = 0;
    let mut inserted = false;
    for num in numbers {
        for x_off in [-1, 0, 1] {
            for y_off in [-1, 0, 1] {
                for x in &num.cols {
                    let x_test = *x as i32 + x_off;
                    let y_test = num.row as i32 + y_off;
                    if x_test >= 0
                        && y_test >= 0
                        && sym_pos.contains(&(x_test as usize, y_test as usize))
                    {
                        result += num.value;
                        inserted = true;
                        break;
                    }
                }
                if inserted {
                    break;
                };
            }
            if inserted {
                break;
            };
        }
        inserted = false;
    }

    result
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap_or(&file_contents);
    let (numbers, symbols) = parse_input(input);
    let gear_pos = symbols
        .iter()
        .filter(|x| x.symbol == '*')
        .map(|x| x.position)
        .collect::<Vec<_>>();

    let mut result: u32 = 0;
    for gear in gear_pos {
        let mut nums: Vec<u32> = vec![];
        for number in &numbers {
            if number.row.abs_diff(gear.1) > 1 {
                continue;
            }
            let mut inserted = false;
            for x in &number.cols {
                for x_off in [-1, 0, 1] {
                    for y_off in [-1, 0, 1] {
                        let x_test = *x as i32 + x_off;
                        let y_test = number.row as i32 + y_off;
                        if x_test >= 0 && y_test >= 0 && gear == (x_test as usize, y_test as usize)
                        {
                            nums.push(number.value);
                            inserted = true;
                            break;
                        }
                    }
                    if inserted {
                        break;
                    };
                }
                if inserted {
                    break;
                };
            }
            if nums.len() > 2 {
                break;
            };
        }
        if nums.len() == 2 {
            result += nums[0] * nums[1];
        }
    }
    result
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
    const TEST_FILENAME: &str = "03.test.txt";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 4361)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 467835)
    }
}
