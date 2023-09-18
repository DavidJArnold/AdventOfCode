use std::collections::HashSet;

fn dist(a: &Vec<i64>, b: &Vec<i64>) -> u64 {
    return a[0].abs_diff(b[0]) + a[1].abs_diff(b[1])
}

fn read_input(contents: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
    let lines = contents.strip_suffix("\n").unwrap_or(contents).split("\n");
    let mut sensors: Vec<(Vec<i64>, Vec<i64>)> = vec![];
    for line in lines {
        let split_line = line.split(" ").collect::<Vec<_>>();
        let idxs = vec![2, 3, 8, 9];
        let data = idxs.iter().map(|i| split_line[*i]);
        let coords = data.map(|x| x.split("=").nth(1).unwrap()).collect::<Vec<_>>();
        let s: Vec<_> = vec![coords[0].strip_suffix(",").unwrap().parse::<i64>().unwrap(), coords[1].strip_suffix(":").unwrap().parse::<i64>().unwrap()];
        let e: Vec<_> = vec![coords[2].strip_suffix(",").unwrap().parse::<i64>().unwrap(), coords[3].parse::<i64>().unwrap()];
        sensors.push((s, e));
    }
    return sensors
}

fn make_circle(centre: &Vec<i64>, radius: i64, range_limit: i64) -> HashSet<(i64, i64)> {
    let radius_list = 0..radius+1;
    let mut perim: HashSet<(i64, i64)> = HashSet::new();
    let offsets: Vec<(i64, i64)> = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for x in radius_list {
        let y = radius - x;
        for offset in &offsets {
            let new_x = centre[0] + offset.0 * x;
            let new_y = centre[1] + offset.1 * y;
            if new_x >= 0 && new_y >= 0 && new_x <= range_limit && new_y <= range_limit {
                perim.insert((new_x, new_y));
            }
        }
    }
    return perim
}

fn find_beacon(range_limit: i64, sensors: Vec<(Vec<i64>, u64)>) -> (i64, i64) {
    for sensor in &sensors {
        let perim = make_circle(&sensor.0, sensor.1 as i64 + 1, range_limit);

        for p in perim {
            let mut valid: bool = true;
            for o in &sensors {
                let d = dist(&vec![p.0, p.1], &o.0);
                if d <= o.1 {
                    valid = false;
                    break;
                }
            }
            if valid {
                return p
            }
        }
    }
    panic!("Couldn't find valid point");
}

fn part1(filename: &str, test_row: i64) -> usize {
    let contents = std::fs::read_to_string(filename).unwrap();
    let data = read_input(&contents);
    let mut blocked_se: HashSet<i64> = HashSet::new();
    let mut blocked: HashSet<i64> = HashSet::new();
    for pair in data {
        let s = pair.0;
        let b = pair.1;
        let dist_sb = dist(&s, &b);
        if b[1] == test_row {
            blocked_se.insert(b[0]);
        }
        if s[1] == test_row {
            blocked_se.insert(s[0]);
        }

        let dist_limit = dist_sb as i64 - s[1].abs_diff(test_row) as i64;
        if dist_limit >= 0 {
            for i in (s[0] - dist_limit)..(s[0] + dist_limit + 1) {
                blocked.insert(i);
            }
        }
    }
    return blocked.len() - blocked_se.len()
}


fn part2(filename: &str, range_limit: i64) -> String {
    let contents = std::fs::read_to_string(filename).unwrap();
    let data = read_input(&contents);
    let sensors = data.iter().map(|x| (x.0.clone(), dist(&x.0, &x.1))).collect::<Vec<_>>();
    let beacon = find_beacon(range_limit, sensors);
    return format!("{}", beacon.0*4_000_000 + beacon.1)
}

fn main() {
    let filename = "15.real.txt";
    let ans1 = part1(&filename, 2_000_000);
    println!("Part 1: {}", ans1);
    let ans2 = part2(&filename, 4_000_000);
    println!("Part 2: {}", ans2);
}
