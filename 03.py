with open("03.real.txt", "r") as f:
    full_str = f.readlines()


def get_priority(ch):
    # calculate priority by converting to ASCII and subtracting an appropriate offset
    if ch.upper() == ch:
        return ord(ch) - 38
    return ord(ch) - 96


s = 0
for rucksack in full_str:
    # split string in half
    midpoint = int(len(rucksack) / 2.0)
    # use sets to get common elements
    in_both = set(rucksack[0:midpoint]) & set(rucksack[midpoint:])
    for ch in in_both:
        # add priorities of all common elements
        s += get_priority(ch)

print("Part 1: ", s)


s2 = 0
while True:
    # get three lines at a time until we can't any more
    # this iterates from the end of the first backwards
    try:
        r1 = full_str.pop().rstrip("\n")
        r2 = full_str.pop().rstrip("\n")
        r3 = full_str.pop().rstrip("\n")
        # use sets to get common elements
        common = set(r1) & set(r2) & set(r3)
        s2 += sum(get_priority(c) for c in common)
    except IndexError:
        # we have finished iterating through the list, it is empty now
        break
print("Part 2: ", s2)
