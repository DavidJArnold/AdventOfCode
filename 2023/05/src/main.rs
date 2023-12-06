const REAL_FILENAME: &str = "05.real.txt";

#[derive(Debug, Clone)]
struct Map {
    sections: Vec<MapSection>,
}

#[derive(Debug, Clone)]
struct MapSection {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let seeds = input
        .split('\n')
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps: Vec<Map> = vec![];
    for map_def in input.split("\n\n").skip(1) {
        let mut map: Vec<MapSection> = vec![];
        for map_section in map_def.split('\n').skip(1) {
            let nums = map_section
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            map.push(MapSection {
                destination_start: nums[0],
                source_start: nums[1],
                length: nums[2],
            });
        }
        maps.push(Map { sections: map });
    }
    (seeds, maps)
}

fn evaluate_map(map: &Map, start: u64) -> u64 {
    for section in &map.sections {
        if (section.source_start..(section.source_start + section.length)).contains(&start) {
            return section.destination_start + start - section.source_start;
        }
    }
    return start;
}

fn evaluate_seeds(seeds: Vec<u64>, maps: &Vec<Map>) -> u64 {
    let mut min_val: u64 = u64::max_value();
    for seed in seeds {
        let mut val = seed;
        for map in maps {
            val = evaluate_map(&map, val);
        }
        min_val = min_val.min(val);
    }
    min_val
}

fn evaluate_seed_inverse(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut val = seed;
    for map in maps.iter().rev() {
        val = evaluate_inverse_map(&map, val);
    }
    val
}

fn part1(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (seeds, maps) = parse_input(input);

    evaluate_seeds(seeds, &maps)
}

fn part2(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (seeds, maps) = parse_input(input);

    let mut min_val: u64 = u64::max_value();
    for seed_idx in 0..seeds.len() / 2 {
        let mut seed_list: Vec<u64> = vec![];
        let range_start = seeds[2 * seed_idx];
        let range_len = seeds[2 * seed_idx + 1];
        for i in 0..range_len {
            seed_list.push(range_start + i);
        }
        min_val = min_val.min(evaluate_seeds(seed_list, &maps));
    }
    min_val
}

fn part2_inverse(filename: &str) -> u64 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let (seeds, maps) = parse_input(input);

    let mut seed_list: Vec<(u64, u64)> = vec![];
    for seed_idx in 0..seeds.len() / 2 {
        let range_start = seeds[2 * seed_idx];
        let range_len = seeds[2 * seed_idx + 1];
        seed_list.push((range_start, range_start + range_len));
    }

    let mut test_val: u64 = 0;
    let mut start = evaluate_seed_inverse(test_val, &maps);
    while !seed_list
        .iter()
        .map(|x| x.0 <= start && x.1 >= start)
        .reduce(|a, b| a || b)
        .unwrap()
    {
        test_val += 1;
        start = evaluate_seed_inverse(test_val, &maps);
    }

    test_val
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2_inverse(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "05.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 35)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 46)
    }

    #[test]
    fn test_part2_inverse() {
        let ans2 = part2_inverse(&TEST_FILENAME);
        assert_eq!(ans2, 46)
    }

    #[test]
    fn test_maps() {
        let map_section_1: MapSection = MapSection {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };
        let map_section_2: MapSection = MapSection {
            destination_start: 52,
            source_start: 50,
            length: 48,
        };
        let map = Map {
            sections: vec![map_section_1, map_section_2],
        };
        assert_eq!(evaluate_map(&map, 98), 50);
        assert_eq!(evaluate_map(&map, 99), 51);
        assert_eq!(evaluate_map(&map, 97), 99);
        assert_eq!(evaluate_map(&map, 53), 55);
    }

    #[test]
    fn test_inverse_maps() {
        let map_section_1: MapSection = MapSection {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };
        let map_section_2: MapSection = MapSection {
            destination_start: 52,
            source_start: 50,
            length: 48,
        };
        let map = Map {
            sections: vec![map_section_1, map_section_2],
        };
        assert_eq!(evaluate_inverse_map(&map, 50), 98);
        assert_eq!(evaluate_inverse_map(&map, 51), 99);
        assert_eq!(evaluate_inverse_map(&map, 99), 97);
        assert_eq!(evaluate_inverse_map(&map, 55), 53);
        assert_eq!(evaluate_inverse_map(&map, 81), 79);
    }
}
