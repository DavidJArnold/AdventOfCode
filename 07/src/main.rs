use std::collections::HashMap;
use std::fs;

fn get_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .trim_end_matches("\n")
        .split("\n")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
}

fn main() {
    let input = get_input("07.test.txt");
    let mut current_path = "/";
    for line in input {
        println!("{:?}", line);
        line.split(" ")
    }
}
