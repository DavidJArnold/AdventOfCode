with open("09.real.txt", "r") as f:
    lines = [x for x in f.read().split("\n")][:-1]


def move_tail(t, h):
    # Move tail, given coordinates of new head position and old tail position
    if h[0] == t[0] and abs(h[1] - t[1]) == 2:
        # vertical move (equal x-pos, y-pos out by 2 -> move up/down 1 spot)
        if h[1] > t[1]:
            t[1] += 1
        else:
            t[1] -= 1
        return t
    elif h[1] == t[1] and abs(h[0] - t[0]) == 2:
        # horizontal move, like vertical
        if h[0] > t[0]:
            t[0] += 1
        else:
            t[0] -= 1
        return t
    elif abs(h[0] - t[0]) + abs(h[1] - t[1]) > 2:
        # diagonal move either (2,1) or (2,2)
        # four possible directions to go
        if h[1] > t[1]:
            t[1] += 1
        else:
            t[1] -= 1
        if h[0] < t[0]:
            t[0] -= 1
        else:
            t[0] += 1
    return t  # if no move is required


record_t = set()  # track unqiue positions using a set
h = [0, 0]  # position (x,y) of head
t = [0, 0]  # position of tail
for line in lines:
    for _ in range(int(line.split(" ")[1])):
        # move head
        if line[0] == "R":
            h[0] += 1
        elif line[0] == "L":
            h[0] -= 1
        elif line[0] == "U":
            h[1] += 1
        elif line[0] == "D":
            h[1] -= 1
        t = move_tail(t, h)
        record_t.add(" ".join([str(x) for x in t]))
print("Part 1: ", len(record_t))

# part 2 basically the same as part 1 but we have many knots to move one-by-one
record_t = set()
num_knots = 10
rope = [[0, 0] for _ in range(num_knots)]
for line in lines:
    for _ in range(int(line.split(" ")[1])):
        if line[0] == "R":
            rope[0][0] += 1
        elif line[0] == "L":
            rope[0][0] -= 1
        elif line[0] == "U":
            rope[0][1] += 1
        elif line[0] == "D":
            rope[0][1] -= 1

        # now move each successive knot
        for idx in range(1, num_knots):
            rope[idx] = move_tail(rope[idx], rope[idx - 1])

        record_t.add(" ".join([str(x) for x in rope[num_knots - 1]]))

print("Part 2: ", len(record_t))
