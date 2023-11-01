const REAL_FILENAME: &str = "";

fn part1(filename: &str) -> () {
}


fn part2(filename: &str) -> () {
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "";
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, ())
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, ())
    }
}
