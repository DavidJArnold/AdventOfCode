with open("02.real.txt", "r") as f:
    full_str = f.readlines()

# Opponent
# A -> Rock
# B -> Paper
# C -> Scissors

# Me
# X -> Rock
# Y -> Paper
# Z -> Scissors


def score(opp, me):  # Part 1
    # scores for playing each shape
    scores = {"X": 1, "Y": 2, "Z": 3}
    # translate opponents codes to my shape codes
    opp_to_me = {"A": "X", "B": "Y", "C": "Z"}

    if me == opp_to_me[opp]:
        # tie
        return scores[me] + 3
    elif (
        (me == "X" and opp_to_me[opp] == "Z")
        or (me == "Y" and opp_to_me[opp] == "X")
        or (me == "Z" and opp_to_me[opp] == "Y")
    ):
        # I win
        return scores[me] + 6
    else:
        # I lose
        return scores[me]


def score2(opp, result):  # Part 2
    scores = {"A": 1, "B": 2, "C": 3}
    # what value beats that key
    winner = {"A": "B", "B": "C", "C": "A"}
    # what value loses to that key
    loser = {"C": "B", "A": "C", "B": "A"}

    if result == "Y":
        # tie
        return scores[opp] + 3
    elif result == "Z":
        # I win
        return scores[winner[opp]] + 6
    else:
        # I lose
        return scores[loser[opp]]


total_score = 0
correct_score = 0
for row in full_str:
    # score each round (using both methods)
    round = row.rstrip("\n").split(" ")
    total_score += score(round[0], round[1])
    correct_score += score2(round[0], round[1])
print("Part 1: ", total_score)
print("Part 2: ", correct_score)
