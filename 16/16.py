import math
from typing import List
from copy import deepcopy
from functools import lru_cache
from time import time


def get_input(filename="16.real.txt"):
    with open(filename, "r") as f:
        return f.read()


class Valve:
    def __init__(self, name, flow_rate):
        self.name = name
        self.flow = flow_rate
        self.connections = []
        self.open = flow_rate == 0

    def add_connection(self, other_valve):
        self.connections.append(other_valve)

    def open_valve(self):
        self.open = True

    def __eq__(self, other):
        return self.name == other.name


class ValveNetwork:
    def __init__(self, valves: List[Valve]):
        self.valves = valves

    def pressure_per_minute(self):
        return sum([valve.flow for valve in self.valves if valve.open])

    def get(self, name: str):
        return [v for v in self.valves if v.name == name][0]

    def open(self, valve: Valve):
        new_network = ValveNetwork(self.valves)
        new_network.get(valve.name).open_valve()
        return new_network


def parse(input):
    connections = {}
    valves = []
    for row in input.split("\n"):
        flow_rate = int(row.split("=", 1)[1].split(";", 1)[0])
        try:
            connections[row[6:8]] = row.split("valves ", 1)[1].split(", ")
        except IndexError:
            connections[row[6:8]] = [row[-2:]]
        valves.append(Valve(name=row[6:8], flow_rate=flow_rate))
    for valve in valves:
        for connection in connections[valve.name]:
            valve.add_connection([V for V in valves if V.name == connection][0])
    return valves


def dijkstra(graph, source):
    dist = []
    prev = []
    Q = []
    for vertex in graph.valves:
        if vertex == source:
            dist.append(0)
        else:
            dist.append(math.inf)
        prev.append(None)
        Q.append(vertex)


# @lru_cache(maxsize=None)
def step(
    time: int,
    valves: ValveNetwork,
    current_valve_name: str,
    value: int,
    max_time: int = 30,
):
    value = value + valves.pressure_per_minute()
    if time == max_time:
        return value

    if all([valve.open for valve in valves.valves]):
        return step(time + 1, valves, current_valve_name, value, max_time)
    # can stay and open a closed valve
    current_valve = valves.get(current_valve_name)
    candidate_vals = []
    if not current_valve.open:
        candidate_vals.append(
            step(
                time + 1,
                valves.open(current_valve),
                current_valve_name,
                value,
                max_time,
            )
        )

    # move to another valve
    for valve in current_valve.connections:
        candidate_vals.append(
            step(time + 1, deepcopy(valves), valve.name, value, max_time)
        )

    return max(candidate_vals)


def part1(filename):
    with open(filename, "r") as f:
        input = f.read().strip()
    print(input)
    valves = parse(input)
    network = ValveNetwork(valves)
    t1 = time()
    print(step(0, network, "AA", 0, 20))
    print(time() - t1)


def part2(filename, val):
    ...


if __name__ == "__main__":
    filename = "16.test.txt"
    p1 = part1(filename)
    print("Part 1:", p1)
    # p2 = part2(filename, val)
    # print("Part 2:", p2)
