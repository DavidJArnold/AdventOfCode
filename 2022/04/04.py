with open("04.real.txt", "r") as f:
    full_str = f.readlines()
s = 0  # for part 1
t = 0  # for part 2
for row in full_str:
    ranges = row.strip("\n").split(",")  # parse input line
    a = ranges[0].split("-")  # first range
    b = ranges[1].split("-")  # second range
    a = [int(x) for x in a]
    b = [int(x) for x in b]

    if (a[0] <= b[0] and a[1] >= b[1]) or (b[0] <= a[0] and b[1] >= a[1]):
        s += 1  # part 1
    if (
        (a[0] <= b[0] and a[1] >= b[0])
        or (a[0] <= b[1] and a[1] >= b[1])
        or (b[0] <= a[0] and b[1] >= a[0])
        or (b[0] <= a[1] and b[1] >= a[1])
    ):
        t += 1  # part 2

print("Part 1: ", s)
print("Part 2: ", t)
