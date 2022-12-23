def get_input(filename="19.real.txt"):
    with open(filename, "r") as f:
        return f.read().strip()


def part1(filename):
    input = get_input(filename)
    total = 0
    for id, blueprint in enumerate(input.split("\n")):
        print(blueprint)
        quality = 1
        total += (id + 1) * quality
    return total


if __name__ == "__main__":
    filename = "19.test.txt"
    p1 = part1(filename)
    print("Part 1:", p1)
    # p2 = part2(filename, val)
    # print("Part 2:", p2)
