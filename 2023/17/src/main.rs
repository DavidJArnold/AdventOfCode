use std::collections::HashMap;

const REAL_FILENAME: &str = "17.real.txt";

#[derive(Debug, PartialEq, Clone)]
struct Node {
    value: i32,
    coords: (i32, i32),
    id: i32,
    connections: Vec<i32>,
    next_dir: Orientation,
}

#[derive(Debug, PartialEq, Clone)]
enum Orientation {
    UpDown,
    LeftRight,
    Any,
}

fn parse_input(input: &str) -> HashMap<(i32, i32), i32> {
    let values: Vec<Vec<i32>> = input
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut field: HashMap<(i32, i32), i32> = HashMap::new();
    for (idx, row) in values.iter().enumerate() {
        for (jdx, val) in row.iter().enumerate() {
            field.insert((jdx.try_into().unwrap(), idx.try_into().unwrap()), *val);
        }
    }
    field
}

// fn is_new(node: &Node, graph: &[Node]) -> bool {
//     !graph.iter().any(|x| {
//         node.coords == x.coords && (x.next_dir == Orientation::Any || node.next_dir == x.next_dir)
//     })
// }

fn get<'a>(graph: &'a [Node], id: &'a i32) -> &'a Node {
    graph.iter().find(|x| x.id == *id).unwrap()
}

fn get_by_coords<'a>(graph: &'a [Node], coords: &'a (i32, i32), dir: Orientation) -> &'a Node {
    graph
        .iter()
        .find(|x| {
            x.coords == *coords
                && (dir == Orientation::Any || x.next_dir == Orientation::Any || x.next_dir == dir)
        })
        .unwrap()
}

fn coord_inside_field(width: i32, height: i32, coord: &(i32, i32)) -> bool {
    coord.0 >= 0 && coord.1 >= 0 && coord.0 <= width && coord.1 <= height
}

fn build_graph(field: &HashMap<(i32, i32), i32>, min_step: i32, max_step: i32) -> Vec<Node> {
    let mut graph: Vec<Node> = Vec::new();

    let width = field.keys().map(|x| x.0).max().unwrap();
    let height = field.keys().map(|x| x.1).max().unwrap();

    // build graph
    graph.push(Node {
        value: *field.get(&(0, 0)).unwrap(),
        coords: (0, 0),
        id: 0,
        connections: vec![],
        next_dir: Orientation::Any,
    });
    graph.push(Node {
        value: *field.get(&(width, height)).unwrap(),
        coords: (width, height),
        id: 1,
        connections: vec![],
        next_dir: Orientation::Any,
    });
    for x in 0..width + 1 {
        for y in 0..height + 1 {
            if (x == 0 && y == 0) || (x == width && y == height) {
                continue;
            }
            let max_id = graph.iter().map(|x| x.id).max().unwrap();
            graph.push(Node {
                value: *field.get(&(x, y)).unwrap(),
                coords: (x, y),
                id: max_id + 1,
                connections: vec![],
                next_dir: Orientation::UpDown,
            });
            graph.push(Node {
                value: *field.get(&(x, y)).unwrap(),
                coords: (x, y),
                id: max_id + 2,
                connections: vec![],
                next_dir: Orientation::LeftRight,
            });
        }
    }

    // add connections
    let graph_nodes = graph.clone();

    for node in &mut graph {
        // Next node is to left/right
        if node.next_dir == Orientation::Any || node.next_dir == Orientation::LeftRight {
            for dist in -max_step..=max_step {
                let next = (node.coords.0 + dist, node.coords.1);
                if (dist < -min_step || dist > min_step) && coord_inside_field(width, height, &next) {
                    node.connections
                        .push(get_by_coords(&graph_nodes, &next, Orientation::UpDown).id);
                }
            }
        }
        // next node is above/below
        if node.next_dir == Orientation::Any || node.next_dir == Orientation::UpDown {
            for dist in -max_step..=max_step {
                let next = (node.coords.0, node.coords.1 + dist);
                if (dist < -min_step || dist > min_step) && coord_inside_field(width, height, &next) {
                    node.connections
                        .push(get_by_coords(&graph_nodes, &next, Orientation::LeftRight).id);
                }
            }
        }
    }

    graph
}

fn get_value(graph: &[Node], start_id: &i32, end_id: &i32) -> i32 {
    let s = get(graph, start_id);
    let e = get(graph, end_id);
    let mut heat: i32 = 0;
    if s.coords.0 == e.coords.0 {
        // vertical
        if s.coords.1 > e.coords.1 {
            for x in e.coords.1..=s.coords.1-1 {
                heat += get_by_coords(graph, &(s.coords.0, x), Orientation::Any).value;
            }
        } else {
            for x in s.coords.1 + 1..=e.coords.1 {
                heat += get_by_coords(graph, &(s.coords.0, x), Orientation::Any).value;
            }
        }
    } else if s.coords.0 > e.coords.0 {
        for x in e.coords.0..=s.coords.0-1 {
            heat += get_by_coords(graph, &(x, s.coords.1), Orientation::Any).value;
        }
    } else {
        for x in s.coords.0 + 1..=e.coords.0 {
            heat += get_by_coords(graph, &(x, s.coords.1), Orientation::Any).value;
        }
    }
    heat
}

fn dijkstra(graph: &[Node], source: &i32, target: &i32) -> i32 {
    let mut q: Vec<i32> = graph.iter().map(|x| x.id).collect::<Vec<i32>>();
    let mut dist: HashMap<i32, i32> = HashMap::new();
    let mut prev: HashMap<i32, i32> = HashMap::new();
    for node in graph {
        let mut this_dist = i32::max_value();
        if &node.id == source {
            this_dist = 0;
        }
        dist.insert(node.id, this_dist);
    }

    while !q.is_empty() {
        let min_dist = q.iter().map(|x| dist.get(x).unwrap()).min().unwrap();
        let q2 = q.clone();
        let u = q2
            .iter()
            .find(|x| dist.get(*x).unwrap() == min_dist)
            .unwrap();
        if target == u {
            // let mut x = u.clone();
            // let mut _path = vec![get(&graph, &x).coords];
            // loop {
            //     x = match prev.get(&x) {
            //         Some(id) => {*id},
            //         None => {break},
            //     };
            //     _path.push(get(&graph, &x).coords);
            // }
            // _path.reverse();
            return *dist.get(u).unwrap();
        }
        let mut remove_idx: usize = 0;
        for (idx, node) in graph.iter().enumerate() {
            if &node.id == u {
                remove_idx = idx;
                break;
            }
        }

        let removed_id = &graph[remove_idx].id;
        let mut q_remove_idx: usize = 0;
        for (idx, node) in q.iter().enumerate() {
            if removed_id == node {
                q_remove_idx = idx;
                break;
            }
        }
        let _ = q.remove(q_remove_idx);

        for v in &graph[remove_idx].connections {
            if !q.contains(v) {
                continue;
            }
            let heat = dist.get(u).unwrap() + get_value(graph, u, v);
            if heat < *dist.get(v).unwrap() {
                dist.insert(*v, heat);
                prev.insert(*v, *u);
            }
        }
    }
    panic!("No path found");
}

fn part1(filename: &str) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    let mut graph = build_graph(&data, 0, 3);
    let all_node_ids: Vec<i32> = graph.iter().map(|x| x.id).collect::<Vec<i32>>();
    for node in &mut graph {
        node.connections.retain(|x| all_node_ids.contains(x));
    }
    let target: i32 = 1;
    let short = dijkstra(&graph, &0, &target);

    short
}

fn part2(filename: &str) -> i32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let data = parse_input(input);
    let mut graph = build_graph(&data, 3, 10);
    let all_node_ids: Vec<i32> = graph.iter().map(|x| x.id).collect::<Vec<i32>>();
    for node in &mut graph {
        node.connections.retain(|x| all_node_ids.contains(x));
    }
    let target: i32 = 1;
    let short = dijkstra(&graph, &0, &target);

    short
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
    const TEST_FILENAME: &str = "17.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 102)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 94)
    }
}
