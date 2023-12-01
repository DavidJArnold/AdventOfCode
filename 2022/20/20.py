from collections import deque


def get_input(filename="20.real.txt"):
    with open(filename, "r") as f:
        return [int(i) for i in f.read().split("\n") if i != ""]


def part1(filename):
    # input is the raw input numbers
    input = get_input(filename)
    # output is a circular list holding indices into input
    output = deque(range(len(input)))
    for idx in range(len(input)):
        # loop over each element, moving the list underneath the element as we go
        # So if we have the list [1, 2, 3, 4, 5, 6] and want to move 3 forwards 2 spaces, we do
        # [1, 2, 4, 5, 6] then move the list back 2 times
        # [2, 4, 5, 6, 1] -> [4, 5, 6, 1, 2]
        # then put the 3 back in where it  was [4, 5, 3, 6, 1, 2]

        # find it's index in the output
        rem = output.index(idx)
        # remove it from the list
        output.remove(idx)
        # rotate the list backwards by the appropriate amount
        output.rotate(-input[idx])
        # put the element back where it was
        output.insert(rem, idx)
    # convert list back to value of input rather than indices of input
    final_list = [input[i] for i in output]
    # find first zero
    first_zero = [i for i, v in enumerate(final_list) if v == 0][0]
    # return the result
    return sum(
        [
            final_list[(first_zero + 1000) % len(final_list)],
            final_list[(first_zero + 2000) % len(final_list)],
            final_list[(first_zero + 3000) % len(final_list)],
        ]
    )


def part2(filename):
    # same as part 1 except each input is multplied by a large number
    # and the shuffling is repeated 10 times
    input = get_input(filename)
    input = [i * 811589153 for i in input]
    output = deque(range(len(input)))
    for _ in range(10):
        for idx in range(len(input)):
            rem = output.index(idx)
            output.remove(idx)
            output.rotate(-input[idx])
            output.insert(rem, idx)
    final_list = [input[i] for i in output]
    first_zero = [i for i, v in enumerate(final_list) if v == 0][0]

    return sum(
        [
            final_list[(first_zero + 1000) % len(final_list)],
            final_list[(first_zero + 2000) % len(final_list)],
            final_list[(first_zero + 3000) % len(final_list)],
        ]
    )


if __name__ == "__main__":
    filename = "20.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
