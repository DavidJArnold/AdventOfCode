use std::fs;
use std::collections::HashSet;

fn get_input(filename: &str) -> String {
    let input = fs::read_to_string(filename).unwrap().trim_end_matches("\n").to_string();
    return input;
}

fn solve(text: &str, window_size: i32) -> i32 {
    let chars = text.chars().collect::<Vec<_>>();
    for (i, el) in chars.windows(window_size as usize).enumerate() {
        let unique_window = el.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
        if unique_window.len() as i32 == window_size {
            return i as i32 + window_size
        }
    }
    return 0
}

fn main() {
    let input = get_input("06.real.txt");
    println!("Part 1: {:?}", solve(&input, 4));
    println!("Part 2: {:?}", solve(&input, 14));
}
