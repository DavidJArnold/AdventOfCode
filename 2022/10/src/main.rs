fn parse_input(text: &str) -> Vec<&str> {
    let contents = text
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect::<Vec<_>>();
    return contents;
}

fn get_values(input: &Vec<&str>) -> Vec<i32> {
    let mut vals: Vec<i32> = [1].to_vec();
    for line in input {
        vals.push(*vals.last().unwrap());
        let op = line.split(" ").collect::<Vec<_>>();
        if op.len() == 2 {
            vals.push(*vals.last().unwrap() + op[1].parse::<i32>().unwrap());
        }
    }
    return vals;
}

fn part1(input: &Vec<&str>) -> i32 {
    get_values(input)
        .into_iter()
        .enumerate()
        .map(|(i, x)| x * i32::try_from(i + 1).unwrap())
        .skip(19)
        .step_by(40)
        .take(6)
        .sum()
}

fn part2(input: &Vec<&str>) -> String {
    let screen_width: i32 = 40;
    let screen_height: i32 = 6;
    let mut screen: Vec<char> = vec![];
    let values = get_values(input);
    for h in 0..screen_height {
        for wid in 0..screen_width {
            let draw_pos = wid + h * screen_width;
            let sprite_pos = vec![-1, 0, 1]
                .iter()
                .map(|x| {
                    values[usize::try_from(draw_pos).unwrap()]
                        + screen_width * (draw_pos / screen_width)
                        + x
                })
                .collect::<Vec<_>>();
            if sprite_pos.contains(&draw_pos) {
                screen.push('#');
            } else {
                screen.push('.');
            }
        }
        screen.push('\n');
    }
    return screen.into_iter().collect();
}

fn main() {
    let file_content = std::fs::read_to_string("10.real.txt").unwrap();
    let input = parse_input(&file_content);
    let ans1 = part1(&input);
    println!("Part 1: {}", ans1);
    let ans2 = part2(&input);
    println!("Part 2:\n{}", ans2);
}
