from copy import deepcopy

with open("18.real.txt", "r") as f:
    input = f.read().strip()

coords = []
for row in input.split("\n"):
    coords.append([int(c) for c in row.split(",")])

matches = 0
# go through each pair of cubes in the input
for cube in coords:
    for other_cube in coords:
        # if two coordinates match, and the third coordinate differ by 1, the cubes are next to each other
        if sum([x == y for (x, y) in zip(cube, other_cube)]) == 2:
            if sum([abs(x - y) for (x, y) in zip(cube, other_cube)]) == 1:
                matches += 1
# Each cube has 6 faces. Subtract the number of matches from this to get the number of exposed faces.
# Note that we count the matches for each face individually, so they are already double-counted.
total1 = 6 * len(coords) - matches
print("Part 1: ", total1)


def find_neighbours(coords, cube_set, m, M):
    # find neighbours of current cube not in coords
    for cube in cube_set:
        # a cube can have 8 neighbours
        dirs = [[0, 0, -1], [0, 0, 1], [0, -1, 0], [0, 1, 0], [-1, 0, 0], [1, 0, 0]]
        for _dir in dirs:
            neighbour = [c + d for (c, d) in zip(cube, _dir)]
            if (
                neighbour not in coords  # not taken up
                and neighbour not in cube_set  # not in the list of cubes
                and all([n <= M + 1 for n in neighbour])  # inside the bounding box
                and all([n >= m - 1 for n in neighbour])
            ):
                cube_set.append(neighbour)
    return cube_set


# The approach is to take a bounding box around all the cubes and
# then find the interior neighbours. Repeat this to find all the
# cubes reachable from the outside. The remaining cubes in the
# bounding box are either filled cubes or empty interior cubes.
# We need to subtract the number of faces of the empty interior
# cubes from the total number of exposed faces.
m = min(
    [
        min([x[0] for x in coords]),
        min([x[1] for x in coords]),
        min([x[2] for x in coords]),
    ]
)
M = max(
    [
        max([x[0] for x in coords]),
        max([x[1] for x in coords]),
        max([x[2] for x in coords]),
    ]
)

# create the bounding box
cube_set = []
for x in range(m - 1, M + 2):
    for y in range(m - 1, M + 2):
        cube_set.append([x, y, m - 1])
        cube_set.append([x, y, M + 1])
for y in range(m - 1, M + 2):
    for z in range(m - 1, M + 2):
        if [m - 1, y, z] not in cube_set:
            cube_set.append([m - 1, y, z])
        if [M + 1, y, z] not in cube_set:
            cube_set.append([M + 1, y, z])
for z in range(m - 1, M + 2):
    for x in range(m - 1, M + 2):
        if [x, m - 1, z] not in cube_set:
            cube_set.append([x, m - 1, z])
        if [x, M + 1, z] not in cube_set:
            cube_set.append([x, M + 1, z])

l = len(cube_set)
while True:
    # keep iterating, adding all the available neighbours to the cube set
    cube_set = find_neighbours(coords, cube_set, m, M)
    if len(cube_set) <= l:
        # if we dind;t add any more cubes, we are finished
        break
    l = len(cube_set)

# now find the interior un-filled cubes
inside_cubes = []
for x in range(m - 1, M + 2):
    for y in range(m - 1, M + 2):
        for z in range(m - 1, M + 2):
            if [x, y, z] not in coords and [x, y, z] not in cube_set:
                inside_cubes.append([x, y, z])

# get the number of faces of the inside cubes
matches = 0
for cube in inside_cubes:
    for other_cube in inside_cubes:
        if sum([x == y for (x, y) in zip(cube, other_cube)]) == 2:
            if sum([abs(x - y) for (x, y) in zip(cube, other_cube)]) == 1:
                matches += 1
total2 = 6 * len(inside_cubes) - matches

print("Part 2: ", total1 - total2)
