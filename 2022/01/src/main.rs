use std::fs;

fn main() {
    let input: String = fs::read_to_string("01.real.txt").expect("Couldn't parse input").strip_suffix("\n").expect("Couldn't find final newline").to_string();

    // sums per group
    let groups = input
        .split("\n\n")
        .map(|x| x.split("\n")
            .map(|y| y.parse::<i32>().unwrap())
            .sum());

    // extract max
    let p1: i32 = groups.clone().max().unwrap();

    // sort groups then take top 3
    let mut p2_groups = groups.collect::<Vec<_>>();
    p2_groups.sort();
    let p2: i32 = p2_groups.iter().rev().take(3).sum();

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}
