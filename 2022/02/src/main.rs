use std::fs;
use std::collections::HashMap;

fn score_part1(pair: Vec<&str>) -> i32 {
    let base_score: i32 = match pair[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let win_map: HashMap<&str, &str> = HashMap::from([("C", "X"), ("B", "Z"), ("A", "Y")]);
    let eq_map: HashMap<&str, &str> = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    
    let score = if pair[1].eq(eq_map.get(pair[0]).unwrap().clone()) {
        3 + base_score
    } else if pair[1].eq(win_map.get(pair[0]).unwrap().clone()) {
        6 + base_score
    } else {
        base_score
    };
    return score
}

fn score_part2(pair: Vec<&str>) -> i32 {
    let base_score: HashMap<&str, i32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let win_map: HashMap<&str, &str> = HashMap::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let lose_map: HashMap<&str, &str> = HashMap::from([("A", "C"), ("B", "A"), ("C", "B")]);
    
    let score = if pair[1] == "Y" {
        3 + base_score.get(pair[0]).unwrap()
    } else if pair[1] == "Z" {
        6 + base_score.get(win_map.get(pair[0]).unwrap()).unwrap()
    } else {
        *base_score.get(lose_map.get(pair[0]).unwrap()).unwrap()
    };
    return score
}

fn main() {
    let input = fs::read_to_string("02.real.txt")
        .expect("Couldn't read input");

    let input_clean = input
        .strip_suffix("\n")
        .expect("Expecting final newline");

    let groups = input_clean
        .split("\n")
        .map(
            |x| x.split(" ").collect::<Vec<&str>>()
        );
    
    let part1: i32 = groups.clone().map(|x| score_part1(x)).sum();
    println!("Part 1: {:?}", part1);
    let part2: i32 = groups.map(|x| score_part2(x)).sum();
    println!("Part 2: {:?}", part2);
}
