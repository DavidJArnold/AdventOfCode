from typing import List, Dict, Any, Union, Tuple
from copy import deepcopy
from time import time


def get_input(filename="16.real.txt") -> str:
    with open(filename, "r") as f:
        return f.read()


def parse(input) -> Tuple[list, list, list, list]:
    flows = []
    names = []
    connections = []
    is_open = []
    for row in input.split("\n"):
        # rows are of the form
        # Valve <ID> has flow rate=<rate>; tunnels lead to valves <ID>, <ID>
        # Sometimes with just a single valve ID at the end and no comma
        flows.append(int(row.split("=", 1)[1].split(";", 1)[0]))
        names.append(row[6:8])
        is_open.append(False)
        try:
            connections.append(row.split("valves ", 1)[1].split(", "))
        except IndexError:  # only a single connection
            connections.append([row[-2:]])
    return flows, names, connections, is_open


def get_spanning_graph(names: list, flows: list, connections: list, keep: List[str]) -> dict:
    # get shortest path between all nodes with non-zero flow rate (plus keep)
    short = {}
    for idx, name in enumerate(names):
        if flows[idx] == 0 and name not in keep:
            continue
        for jdx in range(idx+1, len(names)):
            if name + names[jdx] in short:
                continue
            if flows[jdx] == 0 and names[jdx] not in keep:
                continue
            dist = dijkstra(names, connections, name, names[jdx])
            # print(f"Shortest path from {node} to {other_node} is {dist} long going through {[x for x in prev]}")
            short[name + names[jdx]] = dist
            short[names[jdx] + name] = dist
    return short


def dijkstra(names: list, connections: List[list], source: str, target: str) -> int:
    dist: Dict[str, int] = {}
    Q: list = []
    for vertex in names:
        dist[vertex] = 0 if vertex == source else 1_000_000
        Q.append(vertex)

    while Q is not []:
        min_dist = min(dist[x] for x in Q)
        u = [v for v in Q if dist[v] == min_dist][0]
        if u == target:
            return dist[u]
        idx = [i for i, val in enumerate(names) if val == u][0]
        Q.remove(names[idx])
        for v in connections[idx]:
            if v not in Q:
                continue
            alt = dist[u] + 1
            if alt < dist[v]:
                dist[v] = alt
    raise ValueError("Path not found")


def upper_bound(flows: list, is_open: list, time_left: int, current: int = 0) -> int:
    # get a upper bound on final solution
    release = current
    t = time_left
    while not all(is_open) and t > 0:
        max_flow = 0
        for idx in range(len(flows)):
            if not is_open[idx] and flows[idx] > max_flow:
                max_flow = flows[idx]
                best_vertex = idx
        is_open[best_vertex] = True
        release += (t-2)*max_flow
        t -= 2
    return release


def lower_bound(
    names: list,
    flows: list,
    is_open: list,
    short_paths: Dict[str, int],
    max_time: int,
    pos: str,
    time_left: Union[None, int],
    current: int = 0
) -> int:
    # get a lower bound using the heuristic
    # go to highest value valve, open it, move to the next one
    release = current
    current_vertex = pos
    t = max_time if time_left is None else time_left
    while any(not f for f in is_open) and t > 0:
        max_flow = -1
        best_vertex = None
        best_idx = None
        for idx in range(len(names)):
            if names[idx] == current_vertex:
                continue
            potential = flows[idx] * (t - short_paths[names[idx] + current_vertex] -1)
            if not is_open[idx] and potential > max_flow:
                max_flow = potential
                best_vertex = names[idx]
                best_idx = idx
        if best_vertex is None or best_idx is None:
            return release
        # print(f"Currently have {release}. Opening {best_vertex} and gaining {(t-short_paths[best_vertex + current_vertex]-1)*flows[best_idx]} (aka {max_flow}), will have {t-short_paths[best_vertex + current_vertex]-1} time left afterwards.")
        is_open[best_idx] = True
        travel_time = short_paths[best_vertex + current_vertex]
        current_vertex = best_vertex
        if t - travel_time - 1 <= 0:
            return release
        release += (t-travel_time-1)*flows[best_idx]
        t -= travel_time + 1
    return release


def step2(
    time: int,
    names: Dict[str, int],
    flows: list,
    is_open: list,
    paths: Dict[str, int],
    current_valve_names: List[str] = ["AA", "AA"],
    value: int = 0,
    max_time: int = 30,
    best_release: int = 0
) -> int:
    # runs a DFS to find the highest pressure release

    # recursive base case
    if time >= max_time or all(is_open):
        return value

    candidate_vals = []

    # move to another valve
    for valve_name in names:
        if valve_name == current_valve_name:
            continue
        candidate_idx = names[valve_name]
        arrival_time = time + 1 + paths[current_valve_name + valve_name]
        if arrival_time >= max_time or is_open[candidate_idx]:
            continue
        open_valve = deepcopy(is_open)
        open_valve[candidate_idx] = True
        candidate_vals.append(
            (
                arrival_time,
                names,
                flows,
                open_valve,
                paths,
                valve_name,
                value + (max_time - arrival_time)*flows[candidate_idx],
                max_time
            )
        )

    if len(candidate_vals) == 0:
        return value

    for choice in candidate_vals:
        # check best outcome for this branch
        max_limit = upper_bound(flows, deepcopy(is_open), max_time - choice[0], choice[6])
        if max_limit < best_release:
            continue
        result = step(*choice, best_release=best_release)
        best_release = max(result, best_release)
    return best_release


def step(
    time: int,
    names: Dict[str, int],
    flows: list,
    is_open: list,
    paths: Dict[str, int],
    current_valve_name: str = "AA",
    value: int = 0,
    max_time: int = 30,
    best_release: int = 0
) -> int:
    # runs a DFS to find the highest pressure release

    # recursive base case
    if time >= max_time or all(is_open):
        return value

    candidate_vals = []

    # move to another valve
    for valve_name in names:
        if valve_name == current_valve_name:
            continue
        candidate_idx = names[valve_name]
        arrival_time = time + 1 + paths[current_valve_name + valve_name]
        if arrival_time >= max_time or is_open[candidate_idx]:
            continue
        open_valve = deepcopy(is_open)
        open_valve[candidate_idx] = True
        candidate_vals.append(
            (
                arrival_time,
                names,
                flows,
                open_valve,
                paths,
                valve_name,
                value + (max_time - arrival_time)*flows[candidate_idx],
                max_time
            )
        )

    if len(candidate_vals) == 0:
        return value

    for choice in candidate_vals:
        # check best outcome for this branch
        max_limit = upper_bound(flows, deepcopy(is_open), max_time - choice[0], choice[6])
        if max_limit < best_release:
            continue
        result = step(*choice, best_release=best_release)
        best_release = max(result, best_release)
    return best_release


def prep(filename, max_time: int = 30):
    with open(filename, "r") as f:
        input = f.read().strip()
    flows, names, connections, is_open = parse(input)

    # get minimum distance between relevant points
    # relevant points are starting point, and points with non-zero flow
    short_paths = get_spanning_graph(names, flows, connections, ["AA"])

    # remove irrelevant points from connections, names, flows, etc.
    rem = []
    for idx in range(len(names)):
        if names[idx] != "AA" and flows[idx] == 0:
            # not relevant
            rem.append(idx)
        if names[idx] == "AA":
            # start with AA open, it has 0 flow, so we wouldn't open it later
            is_open[idx] = True
    for it, idx in enumerate(rem):
        connections = [[c for c in d if c != names[idx-it]] for d in connections]
        names.pop(idx-it)
        flows.pop(idx-it)
        is_open.pop(idx-it)

    # get a lower bound on the solution
    # if a proposed path can't beat this, it's not worth checking
    min_rel = lower_bound(names, flows, deepcopy(is_open), short_paths, max_time, 'AA', max_time)

    idx_from_name = {name: idx for idx, name in enumerate(names)}

    return idx_from_name, flows, is_open, short_paths, max_time, min_rel


def part1(filename, max_time: int = 30) -> int:
    idx_from_name, flows, is_open, short_paths, max_time, min_rel = prep(filename, max_time)

    return step(0, idx_from_name, flows, is_open, short_paths, "AA", 0, max_time, min_rel)


def part2(filename: str, max_time: int = 30):
    idx_from_name, flows, is_open, short_paths, max_time, min_rel = prep(filename, max_time)

    return step2(0, idx_from_name, flows, is_open, short_paths, "AA", 0, max_time, min_rel)


if __name__ == "__main__":
    filename = "16.real.txt"
    t0 = time()
    p1 = part1(filename, 30)
    t1 = time()
    print("Part 1:", p1)
    print(f"{t1-t0}s")
    # t0 = time()
    # p2 = part2(filename, 30)
    # t1 = time()
    # print("Part 2:", p2)
    # print(f"{t1-t0}s")
