fn get_input(filename: &str) -> Vec<Vec<u32>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    let size = input.len();
    for (i, x) in input.clone().into_iter().enumerate() {
        for (j, y) in x.clone().into_iter().enumerate() {
            if i == 0 || j == 0 || i == size - 1 || j == size - 1 {
                count += 1;
            } else if y > *x.get(..j).unwrap().into_iter().max().unwrap()
                || y > *x.get(j + 1..).unwrap().into_iter().max().unwrap()
                || y > input.into_iter().map(|x| x[j]).take(i).max().unwrap()
                || y > input.into_iter().map(|x| x[j]).skip(i + 1).max().unwrap()
            {
                count += 1;
            }
        }
    }
    return count;
}

fn count_visible(trees: Vec<u32>, this_tree: u32) -> usize {
    let vis: usize = trees.iter().take_while(|x| *x < &this_tree).count();
    return (vis + 1).min(trees.len());
}

fn part2(input: &Vec<Vec<u32>>) -> usize {
    let mut score: usize = 0;
    let size = input.len();
    for (i, x) in input.clone().into_iter().enumerate() {
        for (j, y) in x.clone().into_iter().enumerate() {
            if !(i == 0 || j == 0 || i == size - 1 || j == size - 1) {
                let mut row_left = x.get(..j).unwrap().to_vec();
                row_left.reverse();
                let row_right = x.get(j + 1..).unwrap().to_vec();
                let row_above = input
                    .clone()
                    .into_iter()
                    .map(|x| x[j])
                    .take(i)
                    .rev()
                    .collect::<Vec<u32>>();
                let row_below = input
                    .clone()
                    .into_iter()
                    .map(|x| x[j])
                    .skip(i + 1)
                    .collect::<Vec<u32>>();
                let l = count_visible(row_left, y);
                let r = count_visible(row_right, y);
                let a = count_visible(row_above, y);
                let b = count_visible(row_below, y);
                score = score.max(a * b * l * r);
            }
        }
    }
    return score;
}

fn main() {
    let input = get_input("08.real.txt");
    let ans1 = part1(&input);
    println!("Part 1: {}", ans1);
    let ans2 = part2(&input);
    println!("Part 2: {}", ans2);
}
