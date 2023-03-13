from datetime import datetime


def parse(filename: str) -> list:
    with open(filename, "r", encoding="utf-8") as file:
        input_text = file.read().strip().split("\n")
    field = []
    for j, row in enumerate(input_text):
        for i, entry in enumerate(row):
            if entry != ".":
                field.append([(i, j), entry])
    return field


def print_field(field, pos=None):
    # Helper function to visualise field area
    field = {f[0]: f[1] for f in field}
    if pos is not None:
        if isinstance(pos, tuple):
            field[pos] = "E"
        if isinstance(pos, dict):
            for p in pos:
                field[p] = "E"
    min_x = min(x[0] for x in field.keys())
    max_x = max(x[0] for x in field.keys())
    min_y = min(x[1] for x in field.keys())
    max_y = max(x[1] for x in field.keys())
    print(
        "\n".join(
            [
                "".join([field.get((i, j), ".") for i in range(min_x, max_x + 1)])
                for j in range(min_y, max_y + 1)
            ]
        )
    )


def move_wind(field: list, width: int, height: int) -> list:
    walls = set(f[0] for f in field if f[1] == "#")
    for idx, elem in enumerate(field):
        new_coord = None
        if elem[1] == ">":
            new_coord = (elem[0][0] + 1, elem[0][1])
            if new_coord in walls:
                new_coord = (new_coord[0] - width, new_coord[1])
        elif elem[1] == "<":
            new_coord = (elem[0][0] - 1, elem[0][1])
            if new_coord in walls:
                new_coord = (new_coord[0] + width, new_coord[1])
        elif elem[1] == "^":
            new_coord = (elem[0][0], elem[0][1] + 1)
            if new_coord in walls:
                new_coord = (new_coord[0], new_coord[1] - height)
        elif elem[1] == "v":
            new_coord = (elem[0][0], elem[0][1] - 1)
            if new_coord in walls:
                new_coord = (new_coord[0], new_coord[1] + height)
        if new_coord is not None:
            field[idx] = [new_coord, elem[1]]
    return field


def part1(filename: str) -> int:
    field = parse(filename)

    W = max(f[0][0] for f in field)
    H = max(f[0][1] for f in field)

    num, _ = traverse(field, (1, 0), (W - 1, H), W, H)
    return num


def part2(filename: str) -> int:
    field = parse(filename)
    W = max(f[0][0] for f in field)
    H = max(f[0][1] for f in field)
    S = (1, 0)
    E = (W - 1, H)
    num_steps1, field = traverse(field, S, E, W, H)
    num_steps2, field = traverse(field, E, S, W, H)
    num_steps3, field = traverse(field, S, E, W, H)
    return num_steps1 + num_steps2 + num_steps3


def is_valid(candidate_pos, blocked, W, H):
    if candidate_pos[0] < 0:
        return False
    if candidate_pos[0] > W:
        return False
    if candidate_pos[1] < 0:
        return False
    if candidate_pos[1] > H:
        return False
    return candidate_pos not in blocked


def traverse(field, start, end, W, H):

    pos = {start: 0}
    idx = 1
    while True:
        field = move_wind(field, W - 1, H - 1)
        blocked = (p[0] for p in field)
        new_pos = {}
        for p in pos:
            for dir_ in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]:
                candidate_pos = (p[0] + dir_[0], p[1] + dir_[1])
                if is_valid(candidate_pos, blocked, W, H):
                    new_pos[candidate_pos] = idx
        pos = new_pos

        if end in pos:
            return idx, field

        idx += 1


if __name__ == "__main__":
    FILE_NAME = "24.real.txt"
    t1 = datetime.now()
    p1 = part1(FILE_NAME)
    t1 = datetime.now() - t1
    t2 = datetime.now()
    p2 = part2(FILE_NAME)
    t2 = datetime.now() - t2

    print(f"Part 1: {p1} ({t1}s)")
    print(f"Part 2: {p2} ({t2}s)")
