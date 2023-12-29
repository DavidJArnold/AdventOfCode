use num::Complex;
use std::collections::HashMap;

#[derive(Debug)]
struct Ray {
    pos: Complex<i32>,
    dir: Complex<i32>,
}

const REAL_FILENAME: &str = "16.real.txt";

fn parse_input(input: &str) -> HashMap<Complex<i32>, char> {
    let mut map: HashMap<Complex<i32>, char> = HashMap::new();
    let height = input.split('\n').count();
    for (idx, line) in input.split('\n').enumerate() {
        for (jdx, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert(
                    Complex {
                        re: jdx as i32,
                        im: height as i32 - idx as i32,
                    },
                    c,
                );
            }
        }
    }
    map
}

fn print_visited(visited: &HashMap<Complex<i32>, Vec<Complex<i32>>>) {
    let mut map_str: Vec<Vec<char>> = vec![];
    let height = visited.keys().map(|x| x.im).max().unwrap();
    let width = visited.keys().map(|x| x.re).max().unwrap();
    for idx in 0..height {
        let mut map_row: Vec<char> = vec![];
        for jdx in -1..width {
            if visited.contains_key(&Complex {
                re: jdx + 1,
                im: idx + 1,
            }) {
                map_row.push('#');
            } else {
                map_row.push('.');
            }
        }
        map_str.push(map_row);
    }
    map_str.reverse();
    for row in map_str {
        println!("{}", row.iter().collect::<String>());
    }
}

fn solve(map: &HashMap<Complex<i32>, char>, start: Ray) -> usize {
    let height = map.keys().map(|x| x.im).max().unwrap();
    let width = map.keys().map(|x| x.re).max().unwrap();
    let mut rays: Vec<Ray> = vec![start];
    let mut visited: HashMap<Complex<i32>, Vec<Complex<i32>>> = HashMap::new();
    visited.insert(rays[0].pos, vec![rays[0].dir]);
    while !rays.is_empty() {
        let mut new_rays: Vec<Ray> = vec![];
        for ray in rays {
            let mut new_dir = vec![ray.dir];
            if map.contains_key(&ray.pos) {
                if ray.dir.re == 0 {
                    // vertical
                    match map.get(&ray.pos).unwrap() {
                        '/' => new_dir[0] *= Complex { re: 0, im: -1 },
                        '\\' => new_dir[0] *= Complex { re: 0, im: 1 },
                        '-' => new_dir = vec![Complex { re: -1, im: 0 }, Complex { re: 1, im: 0 }],
                        _ => {}
                    }
                } else {
                    // horizontal
                    match map.get(&ray.pos).unwrap() {
                        '/' => new_dir[0] *= Complex { re: 0, im: 1 },
                        '\\' => new_dir[0] *= Complex { re: 0, im: -1 },
                        '|' => new_dir = vec![Complex { re: 0, im: 1 }, Complex { re: 0, im: -1 }],
                        _ => {}
                    }
                }
            }
            // move
            for dir in new_dir {
                let next_pos = ray.pos + dir;
                if next_pos.re >= 0
                    && next_pos.re <= width
                    && next_pos.im > 0
                    && next_pos.im <= height
                    && !visited.get(&next_pos).unwrap_or(&vec![]).contains(&dir)
                {
                    new_rays.push(Ray { pos: next_pos, dir });
                    // if visited.contains_key(&next_pos) {
                    //     let dir_list = visited.get_mut(&next_pos).unwrap();
                    //     dir_list.push(dir);
                    // } else {
                    //     visited.insert(next_pos, vec![dir]);
                    // }
                    if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(next_pos) {
                        e.insert(vec![dir]);
                    } else {
                        let dir_list = visited.get_mut(&next_pos).unwrap();
                        dir_list.push(dir);
                    }
                }
            }
        }
        rays = new_rays;
    }
    visited.len()
}

fn part1(filename: &str) -> usize {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let map = parse_input(input);

    let height = map.keys().map(|x| x.im).max().unwrap();
    let start = Ray {
        pos: Complex { re: 0, im: height },
        dir: Complex { re: 1, im: 0 },
    };
    solve(&map, start)
}

fn part2(filename: &str) -> usize {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let map = parse_input(input);

    let height = map.keys().map(|x| x.im).max().unwrap();
    let width = map.keys().map(|x| x.re).max().unwrap();
    // loop over start points and find max value
    let mut max_val = 0;
    for v in 0..height {
        let start = Ray {
            pos: Complex { re: 0, im: v },
            dir: Complex { re: 1, im: 0 },
        };
        max_val = max_val.max(solve(&map, start));
        let start = Ray {
            pos: Complex { re: width, im: v },
            dir: Complex { re: -1, im: 0 },
        };
        max_val = max_val.max(solve(&map, start));
    }
    for h in 0..width {
        let start = Ray {
            pos: Complex { re: h, im: 0 },
            dir: Complex { re: 0, im: 1 },
        };
        max_val = max_val.max(solve(&map, start));
        let start = Ray {
            pos: Complex { re: h, im: height },
            dir: Complex { re: 0, im: -1 },
        };
        max_val = max_val.max(solve(&map, start));
    }
    max_val
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
    const TEST_FILENAME: &str = "16.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 46)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 51)
    }
}
