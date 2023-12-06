import math


def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.read()
    races = []
    race_info = [[int(x) for x in y.split()[1:]] for y in input.split("\n")[:-1]]
    for (time, record) in zip(race_info[0], race_info[1]):
        races.append({"time": time, "record": record})
    return races


def parse_input_2(filename):
    with open(filename, 'r') as file:
        input = file.read()
    race_info = [int("".join(y.split()[1:])) for y in input.split("\n")[:-1]]
    return {"time": race_info[0], "record": race_info[1]}


def run_race(race):
    discriminant = math.sqrt(race["time"]**2 - 4 * race["record"])
    min_win = math.ceil((race["time"] - discriminant) / 2.0)
    max_win = math.floor((race["time"] + discriminant) / 2.0)
    if min_win * (race["time"] - min_win) == race["record"] \
            or max_win * (race["time"] - max_win) == race["record"]:
        return max_win - min_win - 1
    return max_win - min_win + 1


def part1(filename):
    races = parse_input(filename)
    result = 1
    for race in races:
        result *= run_race(race)
    return result


def part2(filename):
    race = parse_input_2(filename)
    return run_race(race)


if __name__ == "__main__":
    filename = "06.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
