use std::collections::VecDeque;

const REAL_FILENAME: &str = "20.real.txt";

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn rotation(input: Vec<i64>, reps: u8) -> VecDeque<usize> {
    let mut output = VecDeque::from_iter(0..input.len());
    for _ in 0..reps {
        for idx in 0..input.len() {
            let rem = output.iter().position(|x| *x == idx).unwrap();
            output.remove(rem);
            if input.get(idx).unwrap() > &0 {
                output.rotate_left(input.get(idx).unwrap().abs() as usize % output.len())
            } else {
                output.rotate_right(input.get(idx).unwrap().abs() as usize % output.len())
            }
            output.insert(rem, idx);
        }
    }
    output
}

fn solve(filename: &str, multiplier: i64, num_rotations: u8) -> i64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let mut input = parse_input(file_contents.trim_end_matches('\n'));
    input = input.iter().map(|x| x * multiplier).collect::<Vec<i64>>();

    let output = rotation(input.clone(), num_rotations);

    let final_list = output.iter().map(|x| input[*x]).collect::<Vec<_>>();
    let first_zero = final_list.iter().position(|x| *x == 0).unwrap();

    final_list[(first_zero + 1_000_usize) % final_list.len()]
        + final_list[(first_zero + 2_000_usize) % final_list.len()]
        + final_list[(first_zero + 3_000_usize) % final_list.len()]
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = solve(filename, 1, 1);
    println!("Part 1: {}", ans1);
    let ans2 = solve(filename, 811589153, 10);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "20.test.txt";
    use std::collections::VecDeque;

    use crate::{parse_input, solve, rotation};

    #[test]
    fn test_rotation() {
        let file_contents = std::fs::read_to_string(TEST_FILENAME).unwrap();
        let input = parse_input(file_contents.trim_end_matches('\n'));

        let output = rotation(input, 1);
        let true_output = VecDeque::from(vec![5, 3, 4, 0, 1, 2, 6]);

        assert_eq!(output, true_output);
    }

    #[test]
    fn test_rotation_10() {
        let file_contents = std::fs::read_to_string(TEST_FILENAME).unwrap();
        let mut input = parse_input(file_contents.trim_end_matches('\n'));
        input = input.iter().map(|x| x * 811589153).collect::<Vec<i64>>();

        let output = rotation(input.clone(), 10);
        let true_output = VecDeque::from(vec![6, 4, 3, 0, 5, 2, 1]);

        assert_eq!(output, true_output);
    }

    #[test]
    fn test_part1() {
        let ans1 = solve(&TEST_FILENAME, 1, 1);
        assert_eq!(ans1, 3)
    }

    #[test]
    fn test_part2() {
        let ans2 = solve(&TEST_FILENAME, 811589153, 10);
        assert_eq!(ans2, 1623178306)
    }
}
