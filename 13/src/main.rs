#[derive(Debug)]
enum VecEntry {
    Number(i32),
    Vector(Vec<VecEntry>),
}

fn read_input(contents: &str) -> Vec<Vec<Vec<VecEntry>>> {
        contents.strip_suffix("\n").unwrap().split("\n\n")
        .map(|x| x.split("\n").map(|y| parse_vec(y)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn parse_vec(string_vec: &str) -> Vec<VecEntry> {
    println!("{:?}", string_vec);
    // TODO: Parse these...
    return vec![VecEntry::Number(0)]
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = read_input(&file_contents);
    println!("{:?}", input);
    return 0
}

fn main() {
    let filename = "13.test.txt";
    let ans1 = part1(&filename);
}
