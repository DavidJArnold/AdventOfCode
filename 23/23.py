from typing import Dict


def parse(filename: str) -> list:
    with open(filename, "r", encoding="utf-8") as file:
        data = file.read().split("\n")
    # parse in a dict with key = identifier and value as a tuple of coordinates
    elves = {}
    idx = 0
    for j, row in enumerate(data):
        for i, spot in enumerate(row):
            if spot == "#":
                # rejig the y coordinate so it increases going upwards not downwards
                elves[idx] = (i, len(data) - j - 1)
                idx += 1
    return elves


def move_elves(elves: Dict[int, tuple], round_num: int):
    proposed: Dict[int, tuple] = {}
    dirs = [
        [(-1, 1), (0, 1), (1, 1)],  # N
        [(-1, -1), (0, -1), (1, -1)],  # S
        [(-1, 1), (-1, 0), (-1, -1)],  # W
        [(1, -1), (1, 0), (1, 1)],  # E
    ]
    around = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

    def add_tuples(a: tuple, b: tuple) -> tuple:
        return tuple(a[i] + b[i] for i in range(2))

    elf_coords = set(elves.values())
    for idx, elf in elves.items():
        proposed[idx] = elf  # current position of this elf
        if len(set(add_tuples(elf, dir) for dir in around) & elf_coords) == 0:
            # nothing surrounding -- stay put
            continue
        for id_ in range(4):
            # go through directions, in different order each time
            dir_idx = (id_ + round_num) % 4
            test_pos = set(add_tuples(elf, dir) for dir in dirs[dir_idx])
            if len(test_pos & elf_coords) == 0:
                # if the test position is free, propose it and move on
                proposed[idx] = add_tuples(elf, dirs[dir_idx][1])
                break

    # initialise output dict
    new: Dict[int, tuple] = {}

    # find duplicated coordinates in proposed positions
    seen = set()
    seen_mult = set(
        coord for coord in proposed.values() if coord in seen or seen.add(coord)
    )
    # get all the elves with duplicated positions
    seen_multiple = set(key for key, coord in proposed.items() if coord in seen_mult)
    # all the rest will move
    not_seen = set(proposed.keys()) - seen_multiple
    for idx in seen_multiple:
        # don't move
        new[idx] = elves[idx]
    for idx in not_seen:
        # move
        new[idx] = proposed[idx]

    return new, elves != new # if elves != new, the configuration has changed


def print_elves(elves):
    """
    Print a display of the area
    """
    min_x = min(x[0] for x in elves.values())
    max_x = max(x[0] for x in elves.values())
    min_y = min(x[1] for x in elves.values())
    max_y = max(x[1] for x in elves.values())
    output = [
        "".join(
            ["#" if (i, j) in elves.values() else "." for i in range(min_x, max_x + 1)]
        )
        for j in range(min_y, max_y + 1)
    ]
    print("\n".join(reversed(output)))


def part1(filename: str) -> int:
    elves = parse(filename)

    for round_num in range(10):
        elves, _ = move_elves(elves, round_num)

    width = max(x[0] for x in elves.values()) - min(x[0] for x in elves.values()) + 1
    height = max(x[1] for x in elves.values()) - min(x[1] for x in elves.values()) + 1

    return width * height - len(elves)


def part2(filename: str) -> int:
    elves = parse(filename)
    round_num = 0
    changes = True
    while changes:
        elves, changes = move_elves(elves, round_num)
        round_num += 1
    return round_num


if __name__ == "__main__":
    FILE_NAME = "23.real.txt"
    p1 = part1(FILE_NAME)
    print("Part 1:", p1)
    p2 = part2(FILE_NAME)
    print("Part 2:", p2)
