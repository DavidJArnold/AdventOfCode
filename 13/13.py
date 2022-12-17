import ast
from functools import cmp_to_key

with open("13.real.txt", "r") as f:
    input = f.read()
    lines = [x for x in input.split("\n\n")]


def equal_greater_smaller(a, b):
    """
    Compares two integers. Returns:
     1 if b>a
     0 if a=b
     -1 if a>b
    """
    return 0 if a == b else (b - a) / abs(b - a)


def check_order(l, r):
    """
    Checks order of two packets. Returns:
     1 if l<r (in order)
     0 if l=r
     -1 if l>r (not in order)
    """
    if isinstance(l, int) and isinstance(r, int):
        # direct comparison, returns 1 if l<r (in order)
        return equal_greater_smaller(l, r)
    elif isinstance(l, list) and isinstance(r, list):
        # iterate through the lists until we can determine the ordering
        for i in range(min(len(l), len(r))):
            c = check_order(l[i], r[i])
            if c == 0:  # inconclusive, keep going
                continue
            return c
        # now check if one list still has items left over, that will be the larger
        return equal_greater_smaller(len(l), len(r))
    elif isinstance(l, list):
        return check_order(l, [r])
    elif isinstance(r, list):
        return check_order([l], r)


c = 0  # track answer
for idx, pair in enumerate(lines):
    # the input is already in python syntax, just evaluate it (safely)
    packets = [ast.literal_eval(p) for p in pair.rstrip("\n").split("\n")]
    in_order = check_order(packets[0], packets[1])
    if in_order == 1:
        c += idx + 1  # add index to answer (1-based)
print("Part 1: ", c)

# parse the input again (slightly different because we don't want pairs of packets)
packet_list = []
for idx, pair in enumerate(lines):
    packets = [ast.literal_eval(p) for p in pair.rstrip("\n").split("\n")]
    packet_list.append(packets[0])
    packet_list.append(packets[1])

# add the special packets
packet_list.append([[2]])
packet_list.append([[6]])

# sort using the comparison function
sorted_packets = sorted(packet_list, key=cmp_to_key(check_order), reverse=True)

# find the special packets
for idx in range(len(sorted_packets)):
    if sorted_packets[idx] == [[2]]:
        c1 = idx + 1
    if sorted_packets[idx] == [[6]]:
        c2 = idx + 1
print("Part 2: ", c1 * c2)
