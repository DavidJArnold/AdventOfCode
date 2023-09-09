fn get_input(filename: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(filename)
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    return contents
    }

fn part1(input: &Vec<String>) -> i32 {
    // let mut vals: Vec<i32> = [1].to_vec();
    for line in input {
        println!("{:?}", line);
        let _ = line.split(" ");
    }
    0
}

fn main() {
    let input = get_input("10.test.txt");
    let ans1 = part1(&input);
    println!("Part 1: {}", ans1);
    // let ans2 = solve(&input, 10);
    // println!("Part 2: {}", ans2);
}
