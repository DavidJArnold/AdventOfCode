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

fn get_dirs(input: Vec<String>) -> HashMap<String, i32> {
    // Build a HashMap of directory locations and their size
    // Then sums up size and adds size of sub-directories
    let mut current_path = "/".to_owned();
    let mut dirs = HashMap::<String, i32>::new();
    for line in input {
        let cmd = line.split(" ").collect::<Vec<_>>();
        if cmd[0] == "$" {
            // It's a command
            if cmd[1] == "cd" {
                if cmd[2] == ".." {
                    // go up a level, remove last entry
                    let (temp, _) = current_path.strip_suffix("/").unwrap().rsplit_once("/").unwrap();
                    current_path = temp.to_string() + "/";
                } else {
                    // enter a directory, append to path
                    if cmd[2] != "/" {
                        current_path.push_str(&(cmd[2].to_owned() + "/"));
                    }
                }
                if !dirs.contains_key(&current_path) {
                    // if we haven't visisted this directory before, add it to the map
                    dirs.insert(current_path.clone(), 0);
                }
            }
        } else {
            // it's not a command, it's a listing output
            if cmd[0] != "dir" {
                // this is a file, so add it's size to the current directory
                let dir_size = dirs.get(&current_path).unwrap().to_owned();
                dirs.insert(current_path.clone(), dir_size + cmd[0].parse::<i32>().unwrap());
            }
        }
    }

    // Now to find the size of the directories including subdirectories, we
    // recursively add the size of sub-directories to each directory
    let mut full_dirs = dirs.clone();
    for key in dirs.keys() {
        for other_key in dirs.keys() {
            if key != other_key && other_key.starts_with(key) {
                let other_dir_size = dirs.get(other_key).unwrap().to_owned();
                let current_dir_size = full_dirs.get(key).unwrap_or(&0).to_owned();
                full_dirs.insert(key.to_owned(), current_dir_size + other_dir_size);
            }
        }
    }
    return full_dirs
}

fn main() {
    let input = get_input("07.real.txt");
    let full_dirs = get_dirs(input);

    let amount: i32 = full_dirs.values().filter(|x| x.to_owned().to_owned() <= 100_000).sum();
    println!("Part 1: {}", amount);

    let need_to_clear = 40_000_000;
    let used_space = full_dirs.get("/").unwrap();
    let freed_space = full_dirs.values().filter(|x| used_space - x.to_owned().to_owned() < need_to_clear).min();
    println!("Part 2: {}", freed_space.unwrap());
}
