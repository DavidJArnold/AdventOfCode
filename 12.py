with open("12.real.txt", "r") as f:
    lines = f.read()


def parse_input(lines, starting_value):
    heights = []
    for line in lines.split("\n")[:-1]:
        h = []
        for c in line:
            if c == "S":
                # starting point -> height 0
                h.append(0)
            elif c == "E":
                # ending point
                h.append(27)
            else:
                # otherwise all lowercase letters so convert from ASCII and offset
                h.append(ord(c) - 96)
        heights.append(h)

    # tracks how many moves required to reach each position
    # starts at -1 everywhere to indicate unreached positions
    moves = [[-1 for _ in range(len(heights[0]))] for _ in range(len(heights))]

    # starting point reachable after 0 moves
    # possibly (part 2) mulitple starting positions
    si, sj = find_vals(heights, starting_value)
    for i, j in zip(si, sj):
        moves[i][j] = 0
    return moves, heights


def find_vals(lists, val):
    """
    Helper function---returns lists of the indices in a list of lists where the value matches val

    Returns lists of indices where lists[I[idx]][J[idx]] == val for all idx in range(len(I))
    """
    I = []
    J = []
    for i, row in enumerate(lists):
        for j, entry in enumerate(row):
            if entry == val:
                I.append(i)
                J.append(j)
    return I, J


def make_move(moves, m_i, m_j, heights, move_set, finished):
    """
    Makes all possible moves at the given location
    """
    if moves[m_i][m_j] != -1:
        curr_height = heights[m_i][m_j]
        for mov in move_set:
            try:  # we get an error on the boundaries
                test_height = heights[m_i + mov[0]][
                    m_j + mov[1]
                ]  # height of candidate position for next move
                if test_height <= curr_height + 1 and (
                    moves[m_i + mov[0]][m_j + mov[1]] == -1
                    or moves[m_i + mov[0]][m_j + mov[1]] > moves[m_i][m_j] + 1
                ):
                    # if the candidate position has appropriate height, and has either not been visited prior,
                    # or took more steps to reach prior, we update it with the appropriate number of moves
                    moves[m_i + mov[0]][m_j + mov[1]] = moves[m_i][m_j] + 1
                    finished = False  # we have changed moves, these changes may propagate on the next iteration
            except IndexError:
                pass
    return moves, finished


def solve(starting_value):
    """Solves part 1 and part 2 (they only have different starting values)"""

    # first parse the input into a list of list of heights at each location
    moves, heights = parse_input(lines, starting_value)

    finished = False  # we only finish after we make no updates on an iteration
    move_set = [  # we can potentially move in 4 directions each step
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0),
    ]
    while not finished:
        finished = True  # finish if this doesn't change

        # iterate through the array, updating the move count as we go
        for m_i in range(len(heights)):
            for m_j in range(len(heights[0])):
                moves, finished = make_move(
                    moves, m_i, m_j, heights, move_set, finished
                )
    # once we are done, find the location of the finishing point, and return the number of moves take to reach
    ei, ej = find_vals(heights, 27)
    return moves[ei[0]][ej[0]]


print("Part 1: ", solve(0))
print("Part 2: ", solve(1))
