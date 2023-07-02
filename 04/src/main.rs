use std::fs;

fn main() {
    let input = fs::read_to_string("04.real.txt").expect("Couldn't read input");
    let input_clean = input.strip_suffix("\n").expect("Expecting final newline");

    let mut s = 0;
    let mut t = 0;
    for row in input_clean.split("\n") {
        let case = row
            .split(['-', ','])
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if ((case[0] <= case[2]) & (case[1] >= case[3]))
            | ((case[2] <= case[0]) & (case[3] >= case[1]))
        {
            s += 1;
        }
        if ((case[0] <= case[2]) & (case[1] >= case[2]))
            | ((case[0] <= case[2]) & (case[1] >= case[3]))
            | ((case[2] <= case[0]) & (case[3] >= case[0]))
            | ((case[2] <= case[1]) & (case[3] >= case[1]))
        {
            t += 1;
        }
    }
    println!("Part 1: {:?}", s);
    println!("Part 2: {:?}", t);
}
