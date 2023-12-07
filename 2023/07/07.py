from functools import cmp_to_key


def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.read().strip()

    card_map = {'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10, '9': 9, '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3 ,'2': 2, '1': 1}

    output = []
    for line in input.splitlines():
        line_contents = line.strip().split()
        hand = [card_map[card] for card in line_contents[0]]
        output.append((hand, int(line_contents[1])))
    return output


def score_hand(hand):
    hand_map = {}
    num_jokers = 0
    for card in hand:
        if card != 1:
            hand_map[card] = hand_map.get(card, 0) + 1
        else:
            num_jokers += 1
    hand_set = list(hand_map.values())
    if len(hand_set) == 0:
        hand_set = [0]
    hand_set.sort(reverse=True)
    hand_set[0] = hand_set[0] + num_jokers
    if hand_set[0] == 5:
        return 6
    elif hand_set[0] == 4:
        return 5
    elif hand_set[0] == 3 and hand_set[1] == 2:
        return 4
    elif hand_set[0] == 3:
        return 3
    elif hand_set[0] == 2 and hand_set[1] == 2:
        return 2
    elif hand_set[0] == 2:
        return 1
    return 0


def cmp(us, other):
    us = us[0]
    other = other[0]
    self_score = score_hand(us)
    other_score = score_hand(other)
    if self_score > other_score:
        return 1
    elif self_score < other_score:
        return -1
    else:
        for idx in range(5):
            if us[idx] > other[idx]:
                return 1
            elif us[idx] < other[idx]:
                return -1
        return 0


def part1(filename):
    data = parse_input(filename)
    data.sort(key=cmp_to_key(cmp))
    return sum((i+1)*x[1] for (i, x) in enumerate(data))


def part2(filename):
    data = parse_input(filename)
    new_data = []
    for entry in data:
        vec = entry[0]
        new_vec = [1 if x == 11 else x for x in vec]
        new_data.append((new_vec, entry[1]))
    new_data.sort(key=cmp_to_key(cmp))
    return sum((i+1)*x[1] for (i, x) in enumerate(new_data))


if __name__ == "__main__":
    filename = "07.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
