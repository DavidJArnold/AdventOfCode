def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.readlines()
    winners = []
    ours = []
    for line in input:
        all_nums = line.strip("\n").split(": ")[1].split(" | ")
        winners.append(set([int(x) for x in all_nums[0].split()]))
        ours.append(set([int(x) for x in all_nums[1].split()]))
    return winners, ours


def part1(filename):
    winners, ours = parse_input(filename)
    total = 0
    for win, us in zip(winners, ours):
        num_winners = len(win.intersection(us))
        total += (num_winners > 0) * 2**(num_winners-1)
    return int(total)


def part2(filename):
    winners, ours = parse_input(filename)
    replicas = [1 for _ in range(len(winners))]
    for idx in range(len(ours)):
        num_winners = len(winners[idx].intersection(ours[idx]))
        for kdx in range(num_winners):
            replicas[idx + 1 + kdx] += replicas[idx]
    return sum(replicas)

if __name__ == "__main__":
    filename = "04.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
