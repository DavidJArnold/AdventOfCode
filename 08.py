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


def count_visible(trees, this_tree):
    # search the list, seeing how far away is the first tree of equal/greater height?
    x = 0
    for tree in trees:
        x += 1
        if tree >= this_tree:
            break
    return x


score = 0  # track maximum
for i in range(1, len(lines) - 1):
    for j in range(1, len(lines) - 1):
        # iterate through each interior tree
        this_tree = lines[i][j]  # height of this tree

        a = count_visible(reversed([lines[idx][j] for idx in range(i)]), this_tree)
        b = count_visible(
            [lines[idx][j] for idx in range(i + 1, len(lines))], this_tree
        )
        l = count_visible(reversed(lines[i][:j]), this_tree)
        r = count_visible(lines[i][j + 1 :], this_tree)

        score = max(score, l * r * a * b)  # track best score

print("Part 2: ", score)
