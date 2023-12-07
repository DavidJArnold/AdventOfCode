import math


def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.read()

    seeds = [int(x) for x in input.split("\n")[0].split()[1:]]

    maps = []
    for map_def in input.split("\n\n")[1:]:
        map = []
        all_nums = [[int(y) for y in x.split()] for x in map_def.strip("\n").split("\n")[1:]]
        for nums in all_nums:
            map.append({"destination_start": nums[0], "source_start": nums[1], "length": nums[2]})
        maps.append(map)
    return seeds, maps


def evaluate_map(map, seed):
    for section in map:
        if section["source_start"] <= seed and seed <= section["source_start"] + section["length"]:
            return section["destination_start"] + seed - section["source_start"]
    return seed


def evaluate_inverse_map(map, seed):
    for section in map:
        if section["destination_start"] <= seed and seed < section["destination_start"] + section["length"]:
            return section["source_start"] + seed - section["destination_start"]
    return seed


def evaluate_seeds(seeds, maps):
    min_val = math.inf
    for seed in seeds:
        val = seed
        for map in maps:
            val = evaluate_map(map, val)
        min_val = min(min_val, val)
    return min_val


def evaluate_seed_inverse(seed, maps):
    for map in maps:
        seed = evaluate_inverse_map(map, seed)
    return seed


def part1(filename):
    (seeds, maps) = parse_input(filename)
    return evaluate_seeds(seeds, maps)


def part2(filename):
    (seeds, maps) = parse_input(filename)
    maps.reverse()

    seed_list = []
    for seed_idx in range(0, len(seeds)//2):
        seed_list.append((seeds[2*seed_idx], seeds[2*seed_idx + 1] + seeds[2*seed_idx]))

    test_val = 0
    start = evaluate_seed_inverse(test_val, maps)
    while not any(x[0] <= start and x[1] >= start for x in seed_list):
        test_val += 1
        start = evaluate_seed_inverse(test_val, maps)
    return test_val


if __name__ == "__main__":
    filename = "05.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
