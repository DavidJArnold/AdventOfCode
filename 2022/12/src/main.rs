fn parse_heights(text: &str) -> Vec<Vec<u32>> {
    let contents = text
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect::<Vec<_>>();
    let mut heights: Vec<Vec<u32>> = vec![];
    for row in contents.iter() {
        let mut row_heights: Vec<u32> = vec![];
        for point in row.chars() {
            match point {
                'S' => row_heights.push(0),
                'E' => row_heights.push(27),
                _ => row_heights.push((point as u32) - 96),
            }
        }
        heights.push(row_heights);
    }
    return heights;
}

fn find(heights: &Vec<Vec<u32>>, value: &u32) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (idx, row) in heights.iter().enumerate() {
        for (jdx, item) in row.iter().enumerate() {
            if item == value {
                output.push((idx, jdx));
            }
        }
    }
    return output;
}

fn parse_moves(heights: &Vec<Vec<u32>>, starting_value: &u32) -> Vec<Vec<i32>> {
    let mut moves = vec![];
    for row in heights {
        let mut r = vec![];
        for _ in row {
            r.push(-1);
        }
        moves.push(r);
    }
    let idxs = find(&heights, starting_value);
    for pos in idxs {
        moves[pos.0][pos.1] = 0;
    }
    return moves;
}

fn solve(starting_value: u32) -> i32 {
    let file_content = std::fs::read_to_string("12.real.txt").unwrap();
    let heights = parse_heights(&file_content);
    let mut moves = parse_moves(&heights, &starting_value);

    let mut finished = false;
    let move_set: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let num_rows = heights.len();
    let num_cols = heights[0].len();
    while !finished {
        finished = true;
        for idx in 0..num_rows {
            for jdx in 0..num_cols {
                let current_height = heights[idx][jdx];
                if moves[idx][jdx] == -1 {
                    continue;
                }
                for mov in &move_set {
                    let candidate = (idx as i32 + mov.0, jdx as i32 + mov.1);
                    if candidate.0 >= 0
                        && candidate.0 < num_rows as i32
                        && candidate.1 >= 0
                        && candidate.1 < num_cols as i32
                    {
                        let test_height = heights[candidate.0 as usize][candidate.1 as usize];
                        let candidate_move = moves[candidate.0 as usize][candidate.1 as usize];
                        if test_height <= current_height + 1
                            && (candidate_move == -1 || candidate_move > moves[idx][jdx] + 1)
                        {
                            moves[candidate.0 as usize][candidate.1 as usize] = moves[idx][jdx] + 1;
                            finished = false;
                        }
                    }
                }
            }
        }
    }
    let end_point = find(&heights, &27);
    return moves[end_point[0].0][end_point[0].1];
}

fn main() {
    let ans1 = solve(0);
    println!("Part 1: {}", ans1);
    let ans2 = solve(1);
    println!("Part 2: {}", ans2);
}
