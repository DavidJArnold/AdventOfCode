import math

with open("17.real.txt", "r") as f:
    input = f.read().strip("\n")


def get_new_piece(index, max_height):
    # creates the coordinates for a new piece
    # index sets the type of piece, and
    # max_height sets its initial position
    if index % 5 == 0:
        # ####
        piece = []
        for i in range(4):
            piece.append((i + 2, max_height + 4))
    elif index % 5 == 1:
        #  #
        # ###
        #  #
        piece = [(3, max_height + 6), (3, max_height + 4)]
        for i in range(3):
            piece.append((i + 2, max_height + 5))
    elif index % 5 == 2:
        #   #
        #   #
        # ###
        piece = [(4, max_height + 6), (4, max_height + 5)]
        for i in range(3):
            piece.append((i + 2, max_height + 4))
    elif index % 5 == 3:
        # #
        # #
        # #
        # #
        piece = [(2, max_height + 4 + i) for i in range(4)]
    elif index % 5 == 4:
        # ##
        # ##
        piece = []
        for i in range(2):
            for j in range(2):
                piece.append((2 + i, max_height + 4 + j))
    return piece


def print_chamber(chamber, piece=None):
    """
    Visualise the chamber, with an optional moving piece
    """
    max_height = 0
    if len(chamber) > 0:
        max_height = max(max_height, max([p[1] for p in chamber]))
    if piece is not None:
        max_height = max([max_height, max([p[1] for p in piece])])
    chamber_vis = [["." for _ in range(7)] for _ in range(max_height + 1)]

    for i in range(max_height + 1):
        for j in range(7):
            if (j, max_height - i) in chamber:
                chamber_vis[i][j] = "#"
            if piece is not None and (j, max_height - i) in piece:
                chamber_vis[i][j] = "@"
    print("\n".join(["".join(c) for c in chamber_vis]))


def run(num_rocks, show_output=False):
    chamber = set()  # record state of the chamber
    index = 0  # the current rock number
    max_height = -1  # current maximum height
    move_idx = 0  # index in the input list of moves

    for index in range(num_rocks):
        # create a piece
        piece = get_new_piece(index, max_height)
        while True:
            # move it until it stops

            if move_idx == len(input):
                move_idx = 0  # reset move_idx to stay in bounds

                if index % 5 == 3 and show_output:
                    # for pattern checking (Part 2)
                    print(index, max_height)

            # do the jet move. get the proposed new position
            if input[move_idx] == ">":
                candidate_position = [(p[0] + 1, p[1]) for p in piece]
            else:
                candidate_position = [(p[0] - 1, p[1]) for p in piece]
            move_idx += 1  # increment move counter

            # now check if it's legal
            if (
                not set(chamber)
                & set(candidate_position)  # doesn't intersect another piece
                and all([p[0] >= 0 for p in candidate_position])  # or the left wall
                and all([p[0] <= 6 for p in candidate_position])  # or the right wall
            ):
                piece = candidate_position

            if all([p[1] > 0 for p in piece]):
                # try moving down
                candidate_position = [(p[0], p[1] - 1) for p in piece]
                if set(candidate_position) & chamber:
                    # can't move down, piece finished moving
                    for p in piece:
                        # add the piece coords to the chamber list
                        chamber.add(p)
                    # update max height
                    max_height = max([p[1] for p in chamber])
                    break
                else:
                    piece = candidate_position
            else:
                for p in piece:
                    chamber.add(p)
                max_height = max([p[1] for p in chamber])
                break

            if index % 50 == 0:
                # every now and again remove far down pieces to save memory
                # 50 seems to work...
                chamber = set(p for p in chamber if p[1] + 50 > max_height)

    return chamber


print("Part 1: ", max([p[1] for p in run(2022)]) + 1)

# For Part 2, we want to identify cycles, so get a long list of results to look at
chamber = run(10_000, False)
# There is a starting period of 1728 rocks, after which we can see a pattern emerge
# where each 1735 rocks, the height increases by 2720.
# So we see how many cycles of 1735 rocks we can fit in the total amount, without getting
# within 3463 rocks of T. We then calculate the remaining (the first rocks) to get the total.

T = 1_000_000_000_000
# Values for test data:
# period = 35
# H = 53
# offset = 43

# for real data:
period = 1735
H = 2720
offset = 1728

N = math.floor((float(T) - offset) / period)

print("Part 2: ", N * H + max([p[1] for p in run((T - offset) % period + offset)]) + 1)
