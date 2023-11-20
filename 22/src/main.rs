use num::complex::Complex;
use std::collections::HashMap;

const REAL_FILENAME: &str = "22.real.txt";
type WrappingFunction = fn(
        &HashMap<Complex<i32>, Tile>,
        Complex<i32>,
        Complex<i32>,
    ) -> (Complex<i32>, Complex<i32>);

#[derive(Debug, PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    turn: Option<TurnDirection>,
    distance: Option<i32>,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Open,
    Closed,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut num_str: String = "".to_string();
    for c in input.chars() {
        if c == 'L' || c == 'R' {
            if num_str != *"" {
                instructions.push(Instruction {
                    turn: None,
                    distance: Some(num_str.parse::<i32>().unwrap()),
                });
                num_str = "".to_string();
            }
            if c == 'L' {
                instructions.push(Instruction {
                    turn: Some(TurnDirection::Left),
                    distance: None,
                });
            } else {
                instructions.push(Instruction {
                    turn: Some(TurnDirection::Right),
                    distance: None,
                });
            }
        } else {
            num_str = format!("{num_str}{c}");
        }
    }
    if num_str != *"" {
        instructions.push(Instruction {
            turn: None,
            distance: Some(num_str.parse::<i32>().unwrap()),
        });
    }
    instructions
}

fn parse_map(input: &str) -> HashMap<Complex<i32>, Tile> {
    let mut map: HashMap<Complex<i32>, Tile> = HashMap::new();

    for (jdx, line) in input.split('\n').enumerate() {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '.' => map.insert(
                    Complex {
                        re: idx as i32 + 1,
                        im: jdx as i32 + 1,
                    },
                    Tile::Open,
                ),
                '#' => map.insert(
                    Complex {
                        re: idx as i32 + 1,
                        im: jdx as i32 + 1,
                    },
                    Tile::Closed,
                ),
                _ => None,
            };
        }
    }

    map
}

fn wrapping_part1(
    map: &HashMap<Complex<i32>, Tile>,
    position: Complex<i32>,
    direction: Complex<i32>,
) -> (Complex<i32>, Complex<i32>) {
    if direction.re == 0 && direction.im == 1 {
        (
            Complex {
                re: position.re,
                im: map
                    .keys()
                    .filter(|x| x.re == position.re)
                    .map(|x| x.im)
                    .min()
                    .unwrap(),
            },
            direction,
        )
    } else if direction.re == 0 && direction.im == -1 {
        (
            Complex {
                re: position.re,
                im: map
                    .keys()
                    .filter(|x| x.re == position.re)
                    .map(|x| x.im)
                    .max()
                    .unwrap(),
            },
            direction,
        )
    } else if direction.re == 1 && direction.im == 0 {
        (
            Complex {
                im: position.im,
                re: map
                    .keys()
                    .filter(|x| x.im == position.im)
                    .map(|x| x.re)
                    .min()
                    .unwrap(),
            },
            direction,
        )
    } else {
        (
            Complex {
                im: position.im,
                re: map
                    .keys()
                    .filter(|x| x.im == position.im)
                    .map(|x| x.re)
                    .max()
                    .unwrap(),
            },
            direction,
        )
    }
}

fn wrapping_part2(
    _: &HashMap<Complex<i32>, Tile>,
    position: Complex<i32>,
    direction: Complex<i32>,
) -> (Complex<i32>, Complex<i32>) {
    let up = Complex { re: 0, im: -1 };
    let down = Complex { re: 0, im: 1 };
    let left = Complex { re: -1, im: 0 };
    let right = Complex { re: 1, im: 0 };
    if position.im == 1 && position.re > 100 && direction == up {
        return (
            Complex {
                re: position.re - 100,
                im: 200,
            },
            up,
        );
    } else if position.re == 150 && direction == right {
        return (
            Complex {
                re: 100,
                im: 151 - position.im,
            },
            left,
        );
    } else if position.re > 100 && position.im == 50 && direction == down {
        return (
            Complex {
                re: 100,
                im: position.re - 50,
            },
            left,
        );
    } else if position.im > 50 && position.im <= 100 && position.re == 100 && direction == right {
        return (
            Complex {
                re: 50 + position.im,
                im: 50,
            },
            up,
        );
    } else if position.re == 100 && position.im >= 101 && direction == right {
        return (
            Complex {
                re: 150,
                im: 151 - position.im,
            },
            left,
        );
    } else if position.im == 150 && position.re > 50 && direction == down {
        return (
            Complex {
                re: 50,
                im: 100 + position.re,
            },
            left,
        );
    } else if position.re == 50 && position.im > 150 && direction == right {
        return (
            Complex {
                re: position.im - 100,
                im: 150,
            },
            up,
        );
    } else if position.im == 200 && direction == down {
        return (
            Complex {
                re: position.re + 100,
                im: 1,
            },
            down,
        );
    } else if position.re == 1 && position.im > 150 && direction == left {
        return (
            Complex {
                re: position.im - 100,
                im: 1,
            },
            down,
        );
    } else if position.re == 1 && position.im <= 150 && direction == left {
        return (
            Complex {
                re: 51,
                im: 151 - position.im,
            },
            right,
        );
    } else if position.im == 101 && position.re <= 50 && direction == up {
        return (
            Complex {
                re: 51,
                im: 50 + position.re,
            },
            right,
        );
    } else if position.re == 51 && position.im > 50 && position.im <= 100 && direction == left {
        return (
            Complex {
                re: position.im - 50,
                im: 101,
            },
            down,
        );
    } else if position.re == 51 && position.im <= 50 && direction == left {
        return (
            Complex {
                re: 1,
                im: 151 - position.im,
            },
            right,
        );
    } else if position.im == 1 && position.re <= 100 && direction == up {
        return (
            Complex {
                re: 1,
                im: 100 + position.re,
            },
            right,
        );
    }
    panic!("Shouldn't be wrapping here?! {position} {direction}");
}

fn solve(
    filename: &str,
    wrapping: WrappingFunction
) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let lines = input.strip_suffix('\n').unwrap();

    let instructions = parse_instructions(lines.split('\n').last().unwrap());

    let map = parse_map(
        &lines
            .split('\n')
            .take(&lines.split('\n').count() - 2)
            .fold(String::new(), |a, b| format!("{a}{b}\n")),
    );

    let up = Complex { re: 0, im: -1 };
    let down = Complex { re: 0, im: 1 };
    let left = Complex { re: -1, im: 0 };
    let right = Complex { re: 1, im: 0 };
    let score: HashMap<Complex<i32>, i32> =
        HashMap::from([(right, 0), (left, 2), (down, 1), (up, 3)]);

    let mut position = Complex {
        re: map
            .keys()
            .filter(|x| x.im == 1)
            .map(|x| x.re)
            .min()
            .unwrap(),
        im: 1,
    };
    let mut direction = right;
    for instruction in instructions {
        if instruction.turn.is_some() {
            match instruction.turn.unwrap() {
                TurnDirection::Left => direction *= Complex { re: 0, im: -1 },
                TurnDirection::Right => direction *= Complex { re: 0, im: 1 },
            }
        }
        if instruction.distance.is_some() {
            for _ in 0..instruction.distance.unwrap() {
                let test_pos = position + direction;
                if map.contains_key(&test_pos) {
                    if map.get(&test_pos).unwrap() == &Tile::Open {
                        position = test_pos;
                    }
                } else {
                    // wrapping
                    let (test_pos, test_dir) = wrapping(&map, position, direction);
                    if map.get(&test_pos).unwrap() == &Tile::Open {
                        position = test_pos;
                        direction = test_dir;
                    }
                }
            }
        }
    }

    1000 * position.im + 4 * position.re + score.get(&direction).unwrap()
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = solve(filename, wrapping_part1);
    println!("Part 1: {}", ans1);
    let ans2 = solve(filename, wrapping_part2);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "22.test.txt";
    use crate::*;

    #[test]
    fn test_input_parsing() {
        let instrs = parse_instructions("10R5L");
        let first_instr = Instruction {
            turn: None,
            distance: Some(10),
        };
        let second_instr = Instruction {
            turn: Some(TurnDirection::Right),
            distance: None,
        };
        let third_instr = Instruction {
            turn: None,
            distance: Some(5),
        };
        let fourth_instr = Instruction {
            turn: Some(TurnDirection::Left),
            distance: None,
        };
        assert_eq!(first_instr, instrs[0]);
        assert_eq!(second_instr, instrs[1]);
        assert_eq!(third_instr, instrs[2]);
        assert_eq!(fourth_instr, instrs[3]);
    }

    #[test]
    fn test_map_parsing() {
        let map = parse_map("  ..#..\n  .#..");
        assert_eq!(map.get(&Complex { re: 1, im: 1 }), None);
        assert_eq!(map.get(&Complex { re: 3, im: 1 }), Some(&Tile::Open));
        assert_eq!(map.get(&Complex { re: 5, im: 1 }), Some(&Tile::Closed));
        assert_eq!(map.get(&Complex { re: 3, im: 2 }), Some(&Tile::Open));
    }

    #[test]
    fn test_part1() {
        let ans1 = solve(&TEST_FILENAME, wrapping_part1);
        assert_eq!(ans1, 6032)
    }
}
