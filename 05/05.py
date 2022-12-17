with open("05.real.txt", "r") as f:
    full_str = f.readlines()

# Example input
#
#     [D]
# [N] [C]
# [Z] [M] [P]
#  1   2   3
#
# move 1 from 2 to 1
# move 3 from 1 to 3
# move 2 from 2 to 1
# move 1 from 1 to 2
#
#
# parse_input decodes this into
#
# [['Z', 'N'], ['M', 'C', 'D'], ['P']]
#
# Note, within each stack the top element is last


def parse_input(full_str: list):

    # first work out where stacks end, the row " 1   2   3"
    # by searching for the 1
    for idx in range(len(full_str)):
        if full_str[idx][1] == "1":
            height = idx
            width = int(full_str[idx][-3])
            break

    # get initial stacks
    stacks = []
    for w in range(width):  # for each stack
        stack = []
        for row in range(height):  # for each row in stack
            # get the letter inside the [ ]
            candidate = full_str[height - row - 1][1 + 4 * w]
            if candidate != " ":
                stack.append(candidate)
        stacks.append(stack)
    return stacks, height, width


def move(move_str: str, stacks: list):
    # move rules for part 1
    moves = move_str.rstrip("\n").split(" ")  # parse movement string
    moved = stacks[int(moves[3]) - 1][-int(moves[1]) :]  # the items to be moved
    stacks[int(moves[3]) - 1] = stacks[int(moves[3]) - 1][
        : -int(moves[1])
    ]  # remove from their old stack
    stacks[int(moves[5]) - 1].extend(reversed(moved))  # add to their new stack
    # because we move in a batch, we have to reverse the list to simulate individual moves
    return stacks


def move2(move_str: str, stacks: list):
    # same as move except we don't have to reverse the moved lists
    moves = move_str.rstrip("\n").split(" ")
    moved = stacks[int(moves[3]) - 1][-int(moves[1]) :]
    stacks[int(moves[3]) - 1] = stacks[int(moves[3]) - 1][: -int(moves[1])]
    stacks[int(moves[5]) - 1].extend((moved))
    return stacks


stacks, height, width = parse_input(full_str)
for row in range(height + 2, len(full_str)):
    # iterate through all moves
    stacks = move(full_str[row], stacks)
print("Part 1: ", "".join([s[-1] for s in stacks]))


stacks, height, width = parse_input(full_str)
for row in range(height + 2, len(full_str)):
    stacks = move2(full_str[row], stacks)
print("Part 2: ", "".join([s[-1] for s in stacks]))
