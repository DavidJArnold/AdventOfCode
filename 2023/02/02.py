def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.readlines()
    games = []
    for line in input:
        rounds = []
        for subset in line.split(":")[1].strip().split(";"):
            counts = {"red": 0, "green": 0, "blue": 0}
            for colour_group in subset.split(","):
                group = colour_group.strip().split(" ")
                counts[group[1]] = int(group[0].lstrip())
            rounds.append(counts)
        games.append(rounds)
    return games


def part1(filename):
    games = parse_input(filename)

    total_cubes = {"red": 12, "green": 13, "blue": 14}
    result = 0
    for (idx, game) in enumerate(games):
        valid = True
        for colour in total_cubes.keys():
            if total_cubes[colour] < max(x[colour] for x in game):
                valid = False
        if valid:
            result += idx + 1
    return result


def part2(filename):
    games = parse_input(filename)
    total = 0
    for game in games:
        power = 1
        for colour in ["red", "green", "blue"]:
            power *= max(x[colour] for x in game)
        total += power
    return total


if __name__ == "__main__":
    filename = "02.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
