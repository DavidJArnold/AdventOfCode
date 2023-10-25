use std::collections::HashMap;

const REAL_FILENAME: &str = "16.real.txt";

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    flow: i32,
    name: String,
    connections: Vec<String>,
    is_open: bool,
}

#[derive(Debug, Clone)]
struct State {
    time: i32,
    valves: Vec<Valve>,
    paths: HashMap<String, i32>,
    valve_name: String,
    value: i32,
    max_time: i32,
}

#[derive(Debug, Clone)]
struct DualState {
    time: i32,
    valves: Vec<Valve>,
    paths: HashMap<String, i32>,
    valve_names: Vec<String>,
    time_next_action: Vec<i32>,
    value: i32,
    max_time: i32,
}

fn parse_input(input: &str) -> Vec<Valve> {
    let mut valves: Vec<Valve> = vec![];

    for line in input.strip_suffix("\n").unwrap().split("\n") {
        let name: String = line.split(" ").nth(1).unwrap().to_owned();
        let flow: i32 = line
            .split(" ")
            .nth(4)
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let connections: Vec<String> = line
            .split(" ")
            .skip(9)
            .map(|x| x.strip_suffix(",").unwrap_or(x).to_owned())
            .collect();
        let mut is_open_status: bool = false;
        if name == "AA" {
            is_open_status = true
        };
        let valve: Valve = Valve {
            name,
            flow,
            connections,
            is_open: is_open_status,
        };
        valves.push(valve);
    }

    return valves;
}

fn get_spanning_graph(valves: &Vec<Valve>, keep: Vec<String>) -> HashMap<String, i32> {
    let mut short: HashMap<String, i32> = HashMap::new();
    for (idx, valve) in valves.iter().enumerate() {
        if !keep.contains(&valve.name) && valve.flow == 0 {
            continue;
        }
        for jdx in (idx + 1)..valves.len() {
            if short
                .keys()
                .collect::<Vec<_>>()
                .contains(&&format!("{}{}", &valve.name, &valves[jdx].name))
            {
                continue;
            }
            if !keep.contains(&valves[jdx].name) && valves[jdx].flow == 0 {
                continue;
            }
            let dist = dijkstra(valves, &valve.name, &valves[jdx].name);
            short.insert(format!("{}{}", valve.name, valves[jdx].name), dist);
            short.insert(format!("{}{}", valves[jdx].name, valve.name), dist);
        }
    }
    return short;
}

fn dijkstra(valves: &Vec<Valve>, source: &String, target: &String) -> i32 {
    let mut q: Vec<&String> = valves.into_iter().map(|x| &x.name).collect();
    let mut dist: HashMap<&String, i32> = HashMap::new();
    for valve in valves {
        let mut this_dist = 1_000_000;
        if &valve.name == source {
            this_dist = 0;
        }
        dist.insert(&valve.name, this_dist);
    }

    while !q.is_empty() {
        let min_dist = q.iter().map(|x| dist.get(*x).unwrap()).min().unwrap();
        let q2 = q.clone();
        let u = q2
            .iter()
            .filter(|x| dist.get(*x).unwrap() == min_dist)
            .nth(0)
            .unwrap();
        if u == &target {
            return *dist.get(u).unwrap();
        }
        let mut remove_idx: usize = 0;
        for idx in 0..valves.len() {
            if &&valves[idx].name == u {
                remove_idx = idx;
                break;
            }
        }

        let removed_name = &valves[remove_idx].name;
        let mut q_remove_idx: usize = 0;
        for idx in 0..q.len() {
            if removed_name == q[idx] {
                q_remove_idx = idx;
                break;
            }
        }
        let _ = q.remove(q_remove_idx);

        for v in &valves[remove_idx].connections {
            if !q.contains(&&v) {
                continue;
            }
            if dist.get(u).unwrap() + 1 < *dist.get(v).unwrap() {
                dist.insert(v, dist.get(u).unwrap() + 1);
            }
        }
    }
    panic!("No path found");
}

fn upper_bound(state: &State, time_left: i32, value: i32) -> i32 {
    let mut new_state = state.clone();
    let mut release = value;
    let mut t = time_left;
    while !new_state.clone().valves.into_iter().all(|x| x.is_open) && t > 2 {
        let mut max_flow = 0;
        let mut best_vertex: Option<&String> = None;
        for valve in &new_state.valves {
            if !valve.is_open && valve.flow > max_flow {
                max_flow = valve.flow;
                best_vertex = Some(&valve.name);
            }
        }
        let best_valve_name = best_vertex.expect("No suitable valve").clone();
        for valve in &mut new_state.valves {
            if valve.name == best_valve_name {
                valve.is_open = true;
                release += (t - 2) * valve.flow;
                t -= 2;
            }
        }
    }
    return release;
}

fn upper_bound2(state: &DualState, time_left: i32, value: i32) -> i32 {
    let mut new_state = state.clone();
    let mut release = value;
    let mut t = time_left;
    while !new_state.clone().valves.into_iter().all(|x| x.is_open) && t > 2 {
        for _ in 0..new_state.valve_names.len() {
            let mut max_flow = 0;
            let mut best_vertex: Option<&String> = None;
            for valve in &new_state.valves {
                if !valve.is_open && valve.flow > max_flow {
                    max_flow = valve.flow;
                    best_vertex = Some(&valve.name);
                }
            }
            if best_vertex == None {
                return release
            }
            let best_valve_name = best_vertex.expect("No suitable valve").clone();
            for valve in &mut new_state.valves {
                if valve.name == best_valve_name {
                    valve.is_open = true;
                    release += (t - 2) * valve.flow;
                }
            }
            t -= 2;
        }
    }
    return release;
}

fn step(state: State, mut best_release: i32) -> i32 {
    if state.time >= state.max_time || state.valves.iter().all(|x| x.is_open) {
        return state.value;
    }

    let mut candidate_vals: Vec<State> = vec![];

    for idx in 0..state.valves.len() {
        let valve = state.valves.get(idx).unwrap();
        if valve.name == state.valve_name {
            continue;
        }
        let mut new_state = state.clone();
        let path_key = format!("{}{}", valve.name, new_state.valve_name);
        let arrival_time = new_state.time + 1 + new_state.paths.get(&path_key).unwrap();
        if arrival_time >= new_state.max_time || valve.is_open {
            continue;
        }

        let current_valve = new_state.valves.get_mut(idx).unwrap();
        current_valve.is_open = true;
        candidate_vals.push(State {
            time: arrival_time,
            valves: new_state.valves,
            paths: new_state.paths,
            valve_name: valve.name.clone(),
            value: new_state.value + (new_state.max_time - arrival_time) * valve.flow.clone(),
            max_time: new_state.max_time,
        })
    }

    if candidate_vals.len() == 0 {
        return state.value;
    }

    for choice in candidate_vals {
        let max_limit = upper_bound(&state, state.max_time - choice.time, choice.value);
        if max_limit < best_release {
            continue;
        }
        let result = step(choice, best_release);
        best_release = result.max(best_release);
    }

    return best_release;
}

fn step2(state: DualState, mut best_release: i32) -> i32 {
    if state.time >= state.max_time || state.valves.iter().all(|x| x.is_open) {
        return state.value;
    }

    let mut candidate_vals: Vec<DualState> = vec![];

    for agent in 0..state.valve_names.len() {
        if state.time_next_action[agent] != state.time {
            continue;
        }

        for idx in 0..state.valves.len() {
            let valve = state.valves.get(idx).unwrap();
            if valve.name == state.valve_names[agent] {
                continue;
            }
            let mut new_state = state.clone();
            let path_key = format!("{}{}", valve.name, new_state.valve_names[agent]);
            let arrival_time = new_state.time + 1 + new_state.paths.get(&path_key).unwrap();
            if arrival_time >= new_state.max_time || valve.is_open {
                continue;
            }

let current_valve = new_state.valves.get_mut(idx).unwrap();
            current_valve.is_open = true;
            // let next_action = new_state.time_next_action.(agent).unwrap();
            // *next_action = arrival_time;
            let _ = std::mem::replace(&mut new_state.time_next_action[agent], arrival_time);
            let _ = std::mem::replace(&mut new_state.valve_names[agent], valve.name.clone());
            candidate_vals.push(DualState {
                time: *new_state.time_next_action.iter().min().unwrap(),
                valves: new_state.valves,
                paths: new_state.paths,
                valve_names: new_state.valve_names,
                time_next_action: new_state.time_next_action,
                value: new_state.value + (new_state.max_time - arrival_time) * valve.flow,
                max_time: new_state.max_time,
            })
        }
    }

    if candidate_vals.len() == 0 {
        return state.value;
    }

    for choice in candidate_vals {
        let max_limit = upper_bound2(&state, state.max_time - choice.time, choice.value);
        if max_limit < best_release {
            continue;
        }
        let result = step2(choice, best_release);
        best_release = result.max(best_release);
    }

    return best_release;
}

fn part1(filename: &str, max_time: i32) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut valves: Vec<Valve> = parse_input(&input);

    let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

    valves = valves
        .into_iter()
        .filter(|x| x.name == "AA" || x.flow != 0)
        .collect::<Vec<Valve>>();

    let starting_state = DualState {
        time: 0,
        valves,
        paths: short_paths,
        valve_names: vec!["AA".to_string()],
        time_next_action: vec![0],
        value: 0,
        max_time,
    };
    let min_rel = 0;

    return step2(starting_state, min_rel);
}

fn part2(filename: &str, max_time: i32) -> i32 {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut valves: Vec<Valve> = parse_input(&input);

    let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

    valves = valves
        .into_iter()
        .filter(|x| x.name == "AA" || x.flow != 0)
        .collect::<Vec<Valve>>();

    let starting_state = DualState {
        time: 0,
        valves,
        paths: short_paths,
        valve_names: vec!["AA".to_string(), "AA".to_string()],
        time_next_action: vec![0, 0],
        value: 0,
        max_time,
    };
    let min_rel = part1(filename, 26);

    return step2(starting_state, min_rel);
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(&filename, 30);
    println!("Part 1: {:?}", ans1);
    let ans2 = part2(&filename, 26);
    println!("Part 2: {:?}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "16.test.txt";
    use super::*;

    #[test]
    fn test_parsing() {
        let filename = TEST_FILENAME;
        let contents = std::fs::read_to_string(filename).unwrap();
        let pipes: Vec<Valve> = parse_input(&contents);

        assert_eq!(
            pipes[0],
            Valve {
                name: "AA".into(),
                flow: 0,
                connections: vec!["DD".into(), "II".into(), "BB".into()],
                is_open: true,
            }
        );

        assert_eq!(
            pipes[7],
            Valve {
                name: "HH".into(),
                flow: 22,
                connections: vec!["GG".into()],
                is_open: false,
            }
        );
    }

    #[test]
    fn test_dijkstra() {
        let filename = TEST_FILENAME;
        let contents = std::fs::read_to_string(filename).unwrap();
        let valves: Vec<Valve> = parse_input(&contents);

        let aa_bb = dijkstra(&valves, &"AA".to_owned(), &"BB".to_owned());
        let jj_aa = dijkstra(&valves, &"JJ".to_owned(), &"AA".to_owned());
        let hh_jj = dijkstra(&valves, &"HH".to_owned(), &"JJ".to_owned());
        let bb_hh = dijkstra(&valves, &"BB".to_owned(), &"HH".to_owned());

        assert_eq!(aa_bb, 1);
        assert_eq!(jj_aa, 2);
        assert_eq!(hh_jj, 7);
        assert_eq!(bb_hh, 6);
    }

    #[test]
    fn test_ub() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = State {
            time: 0,
            valves,
            paths: short_paths,
            valve_name: "AA".to_string(),
            value: 0,
            max_time: 30,
        };

        let ub = upper_bound(
            &starting_state,
            starting_state.max_time,
            starting_state.time,
        );
        assert_eq!(ub, 2024);
    }

    #[test]
    fn test_ub_using_ub2() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = DualState {
            time: 0,
            valves,
            paths: short_paths,
            valve_names: vec!["AA".to_string()],
            time_next_action: vec![0],
            value: 0,
            max_time: 30,
        };

        let ub = upper_bound2(
            &starting_state,
            starting_state.max_time,
            starting_state.time,
        );
        assert_eq!(ub, 2024);
    }

    #[test]
    fn test_ub2() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = DualState {
            time: 0,
            valves,
            paths: short_paths,
            valve_names: vec!["AA".to_string(), "AA".to_string()],
            time_next_action: vec![0, 0],
            value: 0,
            max_time: 4,
        };

        let ub = upper_bound2(
            &starting_state,
            starting_state.max_time,
            starting_state.time,
        );
        assert_eq!(ub, 44);
    }

    #[test]
    fn test_step() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = State {
            time: 0,
            valves,
            paths: short_paths,
            valve_name: "AA".to_string(),
            value: 0,
            max_time: 4,
        };

        let step_result = step(starting_state, 0);
        assert_eq!(step_result, 40);
    }

    #[test]
    fn test_step2_for_part1() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = DualState {
            time: 0,
            valves,
            paths: short_paths,
            valve_names: vec!["AA".to_string()],
            time_next_action: vec![0],
            value: 0,
            max_time: 4,
        };

        let step_result = step2(starting_state, 0);
        assert_eq!(step_result, 40);
    }

    #[test]
    fn test_step_8() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = State {
            time: 0,
            valves,
            paths: short_paths,
            valve_name: "AA".to_string(),
            value: 0,
            max_time: 8,
        };

        let step_result = step(starting_state, 0);
        assert_eq!(step_result, 162);
    }

    #[test]
    fn test_step2() {
        let input = std::fs::read_to_string(&TEST_FILENAME).unwrap();
        let mut valves: Vec<Valve> = parse_input(&input);

        let short_paths = get_spanning_graph(&valves, vec!["AA".into()]);

        valves = valves
            .into_iter()
            .filter(|x| x.name == "AA" || x.flow != 0)
            .collect::<Vec<Valve>>();

        let starting_state = DualState {
            time: 0,
            valves,
            paths: short_paths,
            valve_names: vec!["AA".to_string(), "AA".to_string()],
            time_next_action: vec![0, 0],
            value: 0,
            max_time: 4,
        };

        let step_result = step2(starting_state, 0);
        assert_eq!(step_result, 66);
    }

    #[test]
    fn test_part1() {
        let filename = TEST_FILENAME;
        let ans1 = part1(&filename, 30);
        assert_eq!(ans1, 1651)
    }

    #[test]
    fn test_part2() {
        let filename = TEST_FILENAME;
        let ans2 = part2(&filename, 26);
        assert_eq!(ans2, 1707)
    }
}
