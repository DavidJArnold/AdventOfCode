with open("14.real.txt", "r") as f:
    input = f.read().strip()


def parse_cave(input):
    # represent non-empty points inside the cave by storing them as keys to a dictionary
    cave = {}
    for line in input.split("\n"):
        xy = []
        for coord in line.split(" -> "):
            xy.append([int(z) for z in coord.split(",")])

        for idx in range(len(xy) - 1):
            # join points p1 to p2
            p1 = xy[idx]
            p2 = xy[idx + 1]
            if p1[0] == p2[0]:
                # vertical
                for l in range(abs(p1[1] - p2[1]) + 1):
                    cave[(p1[0], min(p1[1], p2[1]) + l)] = "#"
            if p1[1] == p2[1]:
                # horizontal
                for l in range(abs(p1[0] - p2[0]) + 1):
                    cave[(min(p1[0], p2[0]) + l, p1[1])] = "#"
    return cave


def print_cave(cave):
    # pretty-print the cave dictionary
    pts = [p for p in cave.keys()]
    x_min = min([p[0] for p in pts])
    y_min = min([p[1] for p in pts])
    x_max = max([p[0] for p in pts])
    y_max = max([p[1] for p in pts])
    arr = [["" for _ in range(x_max + 3 - x_min)] for _ in range(y_max + 3 - y_min)]
    for (idx, x) in enumerate(range(x_min - 1, x_max + 2)):
        for (idy, y) in enumerate(range(y_min - 1, y_max + 2)):
            arr[idy][idx] = cave.get((x, y), ".")
    print("\n".join(["".join(row) for row in arr]))


### Part 1
cave = parse_cave(input)
height = max([x[1] for x in cave.keys()])
sand = (0, 0)  # to enter loop
while sand[1] <= height:
    # if any sand goes beyond height, it has fallen off the edge and we stop
    sand = (500, 0)  # starting position for a new sand particle
    while sand[1] <= height:
        # check points below (in order) to see if sand can go there
        if cave.get((sand[0], sand[1] + 1), ".") == ".":
            sand = (sand[0], sand[1] + 1)
        elif cave.get((sand[0] - 1, sand[1] + 1), ".") == ".":
            sand = (sand[0] - 1, sand[1] + 1)
        elif cave.get((sand[0] + 1, sand[1] + 1), ".") == ".":
            sand = (sand[0] + 1, sand[1] + 1)
        else:
            # otherwise it comes to rest, leave a o there
            cave[sand] = "o"
            break
print("Part 1: ", sum([1 for x in cave.values() if x == "o"]))

### Part 2
cave = parse_cave(input)
# add a floor to catch all the sand
height = max([p[1] for p in cave.keys()])
for x in range(-height - 2, height + 3):
    cave[(500 + x, height + 2)] = "#"

sand = (0, 0)
while sand != (500, 0):
    # termination condition slightly different for part 2
    sand = (500, 0)
    while sand[1] <= height + 2:
        if cave.get((sand[0], sand[1] + 1), ".") == ".":
            sand = (sand[0], sand[1] + 1)
        elif cave.get((sand[0] - 1, sand[1] + 1), ".") == ".":
            sand = (sand[0] - 1, sand[1] + 1)
        elif cave.get((sand[0] + 1, sand[1] + 1), ".") == ".":
            sand = (sand[0] + 1, sand[1] + 1)
        else:
            cave[sand] = "o"
            break
print("Part 2: ", sum([1 for x in cave.values() if x == "o"]))
