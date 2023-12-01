def get_first_unique_position(s: str, length: int = 4):
    # iterate through each length "length" substring,
    # determine number of unique elements with a set
    for idx in range(len(s) + 1 - length):
        if len(set(s[idx : idx + length])) == length:
            # all elements unique -> return position
            return idx + length


with open("06.real.txt", "r") as f:
    lines = f.readlines()
    for line in lines:
        s = line.rstrip("\n")
        print("Part 1: ", get_first_unique_position(s))
        print("Part 2: ", get_first_unique_position(s, length=14))
