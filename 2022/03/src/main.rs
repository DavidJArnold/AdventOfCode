use std::fs;

fn priority(x: char) -> i32 {
    if x.is_uppercase() {
        return (x as i32) - 38
    } else {
        return (x as i32) - 96
    }
}

fn main() {
    let input = fs::read_to_string("03.real.txt")
        .expect("Couldn't read input");

    let input_clean = input
        .strip_suffix("\n")
        .expect("Expecting final newline");

    let groups = input_clean
        .split("\n").collect::<Vec<_>>();

    let mut priorities = groups.iter().map(|x| x.chars().map(|y| priority(y)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut total = 0;
    for priority in priorities.clone() {
        let len = priority.len() / 2;
        let left = &priority[0..len];
        let right = &priority[len..];
        let mut both = left.clone().to_owned();
        both.retain(|x| right.contains(x));
        total = total + both[0];
    }
    println!("Part 1: {:?}", total);
    
    let mut total = 0;
    while priorities.len() >= 3 {
        let mut p1 = priorities.pop().unwrap();
        let p2 = priorities.pop().unwrap();
        let p3 = priorities.pop().unwrap();
        p1.retain(|x| p2.contains(x));
        p1.retain(|x| p3.contains(x));
        total += p1[0];
    };
    println!("Part 2: {:?}", total);
}
