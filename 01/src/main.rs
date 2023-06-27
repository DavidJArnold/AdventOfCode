fn main() {
    let contents: String = std::fs::read_to_string("01.real.txt").unwrap();
    let mut sums_array: Vec<i32> = contents.split("\n\n").map(|x| x.split("\n").map(|y| y.parse::<i32>().unwrap_or_else(|_| 0)).sum()).collect::<Vec<i32>>();
    let maximum = sums_array.clone().into_iter().max().unwrap();
    sums_array.sort_by(|a, b| b.cmp(a));
    sums_array.truncate(3);

    println!("Part 1: {}", maximum);
    println!("Part 2: {}", sums_array.into_iter().sum::<i32>());
}
