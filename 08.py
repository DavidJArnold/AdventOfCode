with open("08.real.txt", "r") as f:
    lines = [[int(y, base=10) for y in x] for x in f.read().split("\n")][:-1]

count = 0
for i in range(len(lines)):
    for j in range(len(lines)):
        if i == 0 or j == 0 or i == len(lines) - 1 or j == len(lines) - 1:
            # boundary -> automatically visible
            count += 1
        elif (
            lines[i][j] > max([lines[idx][j] for idx in range(i)])
            or lines[i][j] > max([lines[idx][j] for idx in range(i + 1, len(lines))])
            or lines[i][j] > max(lines[i][:j])
            or lines[i][j] > max(lines[i][j + 1 :])
        ):
            # an internal tree has to be higher than all those in any one direction (left|right|above|below)
            count += 1

print("Part 1: ", count)

score = 0  # track maximum
for i in range(1, len(lines) - 1):
    for j in range(1, len(lines) - 1):
        # iterate through each interior tree
        this_tree = lines[i][j]  # height of this tree

        a = 0  # search above the tree, how far away is the first tree of equal/greater height?
        # reverse the list since it is in order from top-to-bottom
        for tree in reversed([lines[idx][j] for idx in range(i)]):
            a += 1
            if tree >= this_tree:
                break

        b = 0  # below
        for tree in [lines[idx][j] for idx in range(i + 1, len(lines))]:
            b += 1
            if tree >= this_tree:
                break

        l = 0  # left (reversed list since we are looking "backwards" from this tree)
        for tree in reversed(lines[i][:j]):
            l += 1
            if tree >= this_tree:
                break

        r = 0  # right
        for tree in lines[i][j + 1 :]:
            r += 1
            if tree >= this_tree:
                break

        score = max(score, l * r * a * b)  # track best score

print("Part 2: ", score)
