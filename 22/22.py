from copy import deepcopy


def get_input(filename="21.real.txt"):
    with open(filename, "r") as f:
        return f.read().rstrip()


def parse_directions(directions):
    output = []
    num_str = ""
    for c in directions:
        if c == "R" or c == "L":
            if len(num_str) > 0:
                output.append(int(num_str))
                num_str = ""
            output.append(c)
        else:
            num_str += c
    if len(num_str) > 0:
        output.append(int(num_str))
    return output


def print_map(in_map, position, direction):
    icon = {
        complex(0, -1): "^",
        complex(0, 1): "v",
        complex(1, 0): ">",
        complex(-1, 0): "<",
    }
    out_map = deepcopy(in_map)
    temp = list(out_map[int(position.imag) - 1])
    temp[int(position.real) - 1] = icon[direction]
    out_map[int(position.imag) - 1] = "".join(temp)
    print("\n")
    print("\n".join(in_map))


def wrapping_p1(
    map: dict, position: complex, direction: complex
) -> "tuple[complex, complex]":
    if direction == complex(0, 1):
        return (
            complex(
                position.real,
                min([x.imag for x in map.keys() if x.real == position.real]),
            ),
            direction,
        )
    elif direction == complex(0, -1):
        return (
            complex(
                position.real,
                max([x.imag for x in map.keys() if x.real == position.real]),
            ),
            direction,
        )
    elif direction == complex(1, 0):
        return (
            complex(
                min([x.real for x in map.keys() if x.imag == position.imag]),
                position.imag,
            ),
            direction,
        )
    return (
        complex(
            max([x.real for x in map.keys() if x.imag == position.imag]),
            position.imag,
        ),
        direction,
    )


def wrapping_p2(
    map: dict, position: complex, direction: complex
) -> "tuple[complex, complex]":
    # this is hard...
    if direction == complex(0, 1):
        return (
            complex(
                position.real,
                min([x.imag for x in map.keys() if x.real == position.real]),
            ),
            direction,
        )
    elif direction == complex(0, -1):
        return (
            complex(
                position.real,
                max([x.imag for x in map.keys() if x.real == position.real]),
            ),
            direction,
        )
    elif direction == complex(1, 0):
        return (
            complex(
                min([x.real for x in map.keys() if x.imag == position.imag]),
                position.imag,
            ),
            direction,
        )
    return (
        complex(
            max([x.real for x in map.keys() if x.imag == position.imag]),
            position.imag,
        ),
        direction,
    )


def solve(filename, wrapping):
    input = get_input(filename)
    score = {">": 0, "<": 2, "v": 1, "^": 3}
    icon = {
        complex(0, -1): "^",
        complex(0, 1): "v",
        complex(1, 0): ">",
        complex(-1, 0): "<",
    }
    instructions = parse_directions(input.split("\n")[-1])
    in_map = input.split("\n")[:-2]
    map = {}
    for j in range(len(in_map)):
        for i in range(len(in_map[j])):
            if in_map[j][i] == " ":
                continue
            map[complex(i + 1, j + 1)] = in_map[j][i]
    position = complex(min([z.real for z in map.keys() if z.imag == 1]), 1)
    direction = complex(1, 0)
    for instruction in instructions:
        if instruction == "L":
            direction *= complex(0, -1)
        elif instruction == "R":
            direction *= complex(0, 1)
        else:
            for _ in range(instruction):
                test_pos = position + direction
                if test_pos in map.keys():
                    if map[test_pos] != "#":
                        position = test_pos
                else:
                    test_pos, direction = wrapping(map, position, direction)
                    if map[test_pos] != "#":
                        position = test_pos
        # print_map(in_map, position, direction)
        # print(position, direction)
    return int(1000 * position.imag + 4 * position.real + score[icon[direction]])


if __name__ == "__main__":
    filename = "22.real.txt"
    p1 = solve(filename, wrapping_p1)
    print("Part 1:", p1)
    p2 = solve(filename, wrapping_p2)
    print("Part 2:", p2)
