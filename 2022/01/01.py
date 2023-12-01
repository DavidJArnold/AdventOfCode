with open("01.real.txt", "r") as f:
    full_str = f.read()
# split on double newlines to split groups
# then split on newlines within each group, convert to int and sum
sums = [
    sum([int(x) for x in group.split("\n") if x != ""])
    for group in full_str.split("\n\n")
]
print(f"Part 1: {max(sums)}")
print(f"Part 2: {sum(sorted(sums, reverse=True)[0:3])}")
