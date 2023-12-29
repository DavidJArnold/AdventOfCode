const REAL_FILENAME: &str = "15.real.txt";

struct Lens<'a> {
    label: &'a str,
    power: u32,
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect::<Vec<&str>>()
}

fn hash(word: &str) -> u32 {
    word.chars()
        .map(|x| x as u32)
        .fold(0, |x, acc| ((acc + x) * 17) % 256)
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    input
        .strip_suffix('\n')
        .unwrap_or(input)
        .split(',')
        .map(hash)
        .sum::<u32>()
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);

    let mut boxes: Vec<Vec<Lens>> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }
    for lens_spec in data {
        if lens_spec.contains('-') {
            let mut command = lens_spec.split('-');
            let this_label = command.next().unwrap();
            let this_box = boxes.get_mut(hash(this_label) as usize).unwrap();
            let the_idx = this_box.iter().enumerate().find_map(|(idx, x)| {
                if x.label == this_label {
                    Some(idx)
                } else {
                    None
                }
            });
            if let Some(x) = the_idx {
                this_box.remove(x);
            }
        } else if lens_spec.contains('=') {
            let mut command = lens_spec.split('=');
            let this_label = command.next().unwrap();
            let power = command.next().unwrap().parse::<u32>().unwrap();
            let this_box = boxes.get_mut(hash(this_label) as usize).unwrap();
            let the_idx = this_box.iter().enumerate().find_map(|(idx, x)| {
                if x.label == this_label {
                    Some(idx)
                } else {
                    None
                }
            });
            match the_idx {
                Some(x) => {
                    this_box[x] = Lens {
                        label: this_label,
                        power,
                    }
                }
                None => this_box.push(Lens {
                    label: this_label,
                    power,
                }),
            }
        } else {
            panic!("Weird line {lens_spec}");
        };
    }

    boxes
        .iter()
        .enumerate()
        .map(|(idx, a_box)| {
            (idx + 1) as u32
                * a_box
                    .iter()
                    .enumerate()
                    .map(|(jdx, lens)| (jdx + 1) as u32 * lens.power)
                    .sum::<u32>()
        })
        .sum()
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
    const TEST_FILENAME: &str = "15.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 1320)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 145)
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
