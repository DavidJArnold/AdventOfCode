use std::collections::HashSet;

const REAL_FILENAME: &str = "17.real.txt";

fn get_piece(index: usize, max_height: i32) -> HashSet<(i32, i32)> {
    let mut piece: HashSet<(i32, i32)> = HashSet::new();
    match index % 5 {
        0 => {
            for i in 0..4 {
                piece.insert((i + 2, max_height + 3));
            }
        }
        1 => {
            piece.insert((3, max_height + 5));
            piece.insert((3, max_height + 3));
            for i in 0..3 {
                piece.insert((i + 2, max_height + 4));
            }
        }
        2 => {
            piece.insert((4, max_height + 5));
            piece.insert((4, max_height + 4));
            for i in 0..3 {
                piece.insert((i + 2, max_height + 3));
            }
        }
        3 => {
            for i in 0..4 {
                piece.insert((2, max_height + 3 + i));
            }
        }
        4 => {
            for i in 0..2 {
                for j in 0..2 {
                    piece.insert((i + 2, max_height + 3 + j));
                }
            }
        }
        _ => {
            panic!("Unreachable")
        }
    }
    piece
}

fn print_chamber(chamber: &HashSet<(i32, i32)>, new_piece: &HashSet<(i32, i32)>) {
    if chamber.is_empty() && new_piece.is_empty() {
        println!("Chamber empty");
        return;
    }
    let max_height = chamber.iter().map(|x| x.1).max().unwrap_or(0).clone() + 7;
    let mut output: String = "\n".to_string();
    for h in 0..max_height + 1 {
        output += "|";
        for x in 0..7 {
            if chamber.contains(&(x, max_height - h)) {
                output += "#";
            } else if new_piece.contains(&(x, max_height - h)) {
                output += "@";
            } else {
                output += ".";
            }
        }
        output += "|\n";
    }
    output += "+-------+";
    println!("{}", output);
}

fn part1(filename: &str, num_rocks: usize) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut jet_cycle = input.strip_suffix("\n").unwrap().chars().cycle();
    let mut chamber: HashSet<(i32, i32)> = HashSet::new();

    for rock_index in 0..num_rocks {
        let mut piece = get_piece(rock_index, chamber.iter().map(|x| x.1+1).max().unwrap_or(0));
        // println!("\n{:?}", chamber);
        // print_chamber(&chamber, &piece);

        loop {
            // try moving from jet
            let direction = match jet_cycle.next() {
                Some('>') => 1,
                Some('<') => -1,
                _ => panic!("Unexpected token"),
            };
            let mut new_piece = piece.iter().map(|x| (x.0 + direction, x.1)).collect::<HashSet<_>>();

            let valid = chamber.is_disjoint(&new_piece)
                && new_piece.iter().map(|x| x.0).min().unwrap() >= 0
                && new_piece.iter().map(|x| x.0).max().unwrap() <= 6;
            if !valid {
                new_piece = piece;
            }

            // try moving down
            let down_piece: HashSet<(i32, i32)> = new_piece.iter().map(|x| (x.0, x.1 - 1)).collect::<HashSet<_>>();
            if !down_piece.is_disjoint(&chamber)
                || down_piece.iter().map(|x| x.1).min().unwrap() == -1
            {
                let _ = chamber.extend(&new_piece);
                break;
            } else {
                piece = down_piece.clone();
            }
        }

        if rock_index % 50 == 0 {
            let max_height = chamber.iter().map(|x| x.1).max().unwrap().clone();
            chamber = chamber.iter().filter(|x| x.1 > max_height - 50).map(|x| x.to_owned()).collect::<HashSet<_>>();
        }
    }

    chamber.iter().map(|x| x.1).max().unwrap().clone() + 1
}

fn part2(filename: &str) -> i64 {
    
    // For Part 2, we identify cycles, so get a long list of results to look at
    // There is a starting period of 1728 rocks, after which we can see a pattern emerge
    // where each 1735 rocks, the height increases by 2720.
    // So we see how many cycles of 1735 rocks we can fit in the total amount, without getting
    // within 3463 rocks of T. We then calculate the remaining (the first rocks) to get the total.

    const T: usize = 1_000_000_000_000;
    // Values for test data:
    // period = 35
    // H = 53
    // offset = 43

    // for real data:
    const PERIOD: usize = 1735;
    const H: usize = 2720;
    const OFFSET: usize = 1728;

    let n = ((T as f64 - OFFSET as f64) / PERIOD as f64).floor() as usize;

    (n * H) as i64 + part1(&filename, (T - OFFSET) % PERIOD + OFFSET) as i64
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename, 2022);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    use crate::part1;
    const TEST_FILENAME: &str = "17.test.txt";

    #[test]
    fn test_part1_1() {
        let ans1 = part1(TEST_FILENAME, 1);
        assert_eq!(ans1, 1)
    }

    #[test]
    fn test_part1_2() {
        let ans1 = part1(TEST_FILENAME, 2);
        assert_eq!(ans1, 4)
    }

    #[test]
    fn test_part1_10() {
        let ans1 = part1(TEST_FILENAME, 10);
        assert_eq!(ans1, 17)
    }

    #[test]
    fn test_part1_20() {
        let ans1 = part1(TEST_FILENAME, 20);
        assert_eq!(ans1, 36)
    }

    #[test]
    fn test_part1_50() {
        let ans1 = part1(TEST_FILENAME, 50);
        assert_eq!(ans1, 78)
    }

    #[test]
    fn test_part1() {
        let ans1 = part1(TEST_FILENAME, 2022);
        assert_eq!(ans1, 3068)
    }
}
