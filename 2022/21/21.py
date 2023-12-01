def get_input(filename="21.real.txt"):
    with open(filename, "r") as f:
        return f.read().strip()


def part1(filename):
    input = get_input(filename)
    monkeys = {}
    for line in input.split("\n"):
        splitline = line.split(": ")
        # parse input as either a number or an operation
        try:
            monkeys[splitline[0]] = int(splitline[1])
        except ValueError:
            monkeys[splitline[0]] = splitline[1]
    while True:
        for monkey in monkeys.keys():
            # find each monkey's value
            # keep looping untili we have all the values
            if isinstance(monkeys[monkey], (int, float)):
                continue

            operation = monkeys[monkey].split(" ")
            if isinstance(monkeys[operation[0]], (int, float)) and isinstance(
                monkeys[operation[2]], (int, float)
            ):
                # we have neough information to evaluate this monkey
                if operation[1] == "+":
                    monkeys[monkey] = monkeys[operation[0]] + monkeys[operation[2]]
                elif operation[1] == "-":
                    monkeys[monkey] = monkeys[operation[0]] - monkeys[operation[2]]
                elif operation[1] == "*":
                    monkeys[monkey] = monkeys[operation[0]] * monkeys[operation[2]]
                elif operation[1] == "/":
                    monkeys[monkey] = monkeys[operation[0]] // monkeys[operation[2]]
        if isinstance(monkeys["root"], (int, float)):
            # finish once we know all the values
            break
    return monkeys["root"]


def part2(filename, val):
    # with a given value of the specified monkey, find the error in the equality statement for root
    input = get_input(filename)
    monkeys = {}
    for line in input.split("\n"):
        splitline = line.split(": ")
        try:
            monkeys[splitline[0]] = int(splitline[1])
        except ValueError:
            monkeys[splitline[0]] = splitline[1]
    ROOT = monkeys["root"].split(" ")
    monkeys["root"] = " ".join(
        [ROOT[0], "=", ROOT[2]]
    ).strip()  # replace this operation
    monkeys["humn"] = val  # replace this value
    while True:
        for monkey in monkeys.keys():
            if isinstance(monkeys[monkey], (int, float)):
                continue
            operation = monkeys[monkey].split(" ")
            if isinstance(monkeys[operation[0]], (int, float)) and isinstance(
                monkeys[operation[2]], (int, float)
            ):
                if operation[1] == "+":
                    monkeys[monkey] = monkeys[operation[0]] + monkeys[operation[2]]
                elif operation[1] == "-":
                    monkeys[monkey] = monkeys[operation[0]] - monkeys[operation[2]]
                elif operation[1] == "*":
                    monkeys[monkey] = monkeys[operation[0]] * monkeys[operation[2]]
                elif operation[1] == "/":
                    monkeys[monkey] = monkeys[operation[0]] / monkeys[operation[2]]
        if all(
            [
                isinstance(monkeys[m], (int, float))
                for m in monkeys.keys()
                if m != "root"
            ]
        ):
            # root will never be an int because we don't calculate it
            break

    # test is the error in the values -- this should be zero
    test = (
        monkeys[monkeys["root"].split(" ")[0]] - monkeys[monkeys["root"].split(" ")[2]]
    )
    return test, monkeys["humn"]


def solve(filename, start=1000000000000):
    # simple root finding algorithm. Starts with large jumps, decreases when serach direction changes
    # Assumes function is monotonic decreasing
    test = 1
    step = 100000000000
    last_op = None
    while test != 0:
        test, result = part2(filename, start)
        if test > 0:
            if last_op == "Down":
                step = max(1, step // 10)
            start += step
            last_op = "Up"
        elif test < 0:
            if last_op == "Up":
                step = max(1, step // 10)
            start -= step
            last_op = "Down"
        else:
            return result


if __name__ == "__main__":
    filename = "21.real.txt"
    p1 = part1(filename)
    print("Part 1:", p1)
    p2 = solve(filename)
    print("Part 2:", p2)
