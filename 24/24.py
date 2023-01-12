from copy import deepcopy


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
    print("\n".join(["".join([field.get((i, j), ".") for i in range(min_x, max_x+1)]) for j in range(min_y,max_y+1)]))

def add_tuple(a, b):
    """Adds two tuples element-wise"""
    return tuple(a[i]+b[i] for i in range(2))

dirs = {
    ">": (1, 0),
    "<": (-1, 0),
    "^": (0, -1),
    "v": (0, 1)
}

def move_wind(field: list) -> list:
    width = max(f[0][0] for f in field) - min(f[0][0] for f in field) - 1
    height = max(f[0][1] for f in field) - min(f[0][1] for f in field) - 1
    
    walls = set(f[0] for f in field if f[1] == "#")
    for idx, elem in enumerate(field):
        if elem[1] in dirs:
            new_coord = add_tuple(elem[0], dirs[elem[1]])
            if new_coord in walls:
                if elem[1] in (">", "<"):
                    size_ = width
                elif elem[1] in ("^", "v"):
                    size_ = height
                new_coord = add_tuple(new_coord, tuple(-size_*s for s in dirs[elem[1]]))
            field[idx] = [new_coord, elem[1]]
    return field

def part1(filename: str) -> int:
    field = parse(filename)
    W = max(f[0][0] for f in field)
    H = max(f[0][1] for f in field)
    num, _ = traverse(field, (1, 0), (W-1, H))
    return num

def part2(filename: str) -> int:
    field = parse(filename)
    W = max(f[0][0] for f in field)
    H = max(f[0][1] for f in field)
    S = (1, 0)
    E = (W-1, H)
    num_steps1, field = traverse(field, S, E)
    num_steps2, field = traverse(field, E, S)
    num_steps3, field = traverse(field, S, E)
    return num_steps1 + num_steps2 + num_steps3
def is_valid(candidate_pos, blocked, W, H):
    return candidate_pos not in blocked and \
        candidate_pos[0] >= 0 and \
            candidate_pos[0] <= W and \
                candidate_pos[1] >= 0 and \
                    candidate_pos[1] <= H

def traverse(field, start, end):
    W = max(f[0][0] for f in field)
    H = max(f[0][1] for f in field)
    
    pos = {start: 0}
    idx = 1
    while True:
        field = move_wind(field)
        blocked = set(p[0] for p in field)
        new_pos = {}
        for p in pos:
            for dir_ in [(0,0), (1,0), (-1,0), (0,1), (0,-1)]:
                candidate_pos = add_tuple(p, dir_)
                if is_valid(candidate_pos, blocked, W, H):
                    new_pos[candidate_pos] = idx
        pos = new_pos

        if end in pos:
            return idx, field

        idx += 1


if __name__ == "__main__":
    FILE_NAME = "24.real.txt"
    p1 = part1(FILE_NAME)
    print("Part 1:", p1)
    p2 = part2(FILE_NAME)
    print("Part 2:", p2)