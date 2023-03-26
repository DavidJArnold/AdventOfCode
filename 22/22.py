from typing import Callable, Tuple
from time import time


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


def wrapping_p1(
    map: dict, position: complex, direction: complex
) -> Tuple[complex, complex]:
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
    _: dict, position: complex, direction: complex
) -> Tuple[complex, complex]:
    up = complex(0, -1)
    down = complex(0, 1)
    left = complex(-1, 0)
    right = complex(1, 0)
    if position.imag == 1 and position.real > 100 and direction == up:
        # 1ab
        return (complex(position.real - 100, 200), up)
    elif position.real == 150 and direction == right:
        # 1bc
        return (complex(100, 151 - position.imag), left)
    elif position.real > 100 and position.imag == 50 and direction == down:
        # 1cb
        return (complex(100, position.real - 50), left)
    elif position.imag > 50 and position.imag <= 100 and position.real == 100 and direction == right:
        # 3bc
        return (complex(50 + position.imag, 50), up)
    elif position.real == 100 and position.imag >= 101 and direction == right:
        # 4bc
        return (complex(150, 151 - position.imag), left)
    elif position.imag == 150 and position.real > 50 and direction == down:
        # 4dc
        return (complex(50, 100 + position.real), left)
    elif position.real == 50 and position.imag > 150 and direction == right:
        # 6bc
        return (complex(position.imag - 100, 150), up)
    elif position.imag == 200 and direction == down:
        # 6dc
        return (complex(position.real + 100, 1), down)
    elif position.real == 1 and position.imag > 150 and direction == left:
        # 6ad
        return (complex(position.imag - 100, 1), down)
    elif position.real == 1 and position.imag <= 150 and direction == left:
        # 5ad
        return (complex(51, 151 - position.imag), right)
    elif position.imag == 101 and position.real <= 50 and direction == up:
        # 5ab
        return (complex(51, 50 + position.real), right)
    elif position.real == 51 and position.imag > 50 and position.imag <= 100 and direction == left:
        # 3ad
        return (complex(position.imag - 50, 101), down)
    elif position.real == 51 and position.imag <= 50 and direction == left:
        # 2ad
        return (complex(1, 151 - position.imag), right)
    elif position.imag == 1 and position.real <= 100 and direction == up:
        # 2ab
        return (complex(1, 100 + position.real), right)

    raise ValueError(f"Couldn't wrap at {position} with direction {direction}")


def solve(
    filename: str,
    wrapping: Callable[[dict, complex, complex], Tuple[complex, complex]]
) -> int:
    text_input = get_input(filename)
    up = complex(0, -1)
    down = complex(0, 1)
    left = complex(-1, 0)
    right = complex(1, 0)
    score = {right: 0, left: 2, down: 1, up: 3}
    instructions = parse_directions(text_input.split("\n")[-1])
    in_map = text_input.split("\n")[:-2]
    map = {}
    for j in range(len(in_map)):
        for i in range(len(in_map[j])):
            if in_map[j][i] == " ":
                continue
            map[complex(i + 1, j + 1)] = in_map[j][i]
    position = complex(min([z.real for z in map.keys() if z.imag == 1]), 1)
    direction = right
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
                    test_pos, test_dir = wrapping(map, position, direction)
                    if map[test_pos] != "#":
                        position = test_pos
                        direction = test_dir
    return int(1000 * position.imag + 4 * position.real + score[direction])


if __name__ == "__main__":
    filename = "22.real.txt"
    t1 = time()
    p1 = solve(filename, wrapping_p1)
    t2 = time()
    print("Part 1:", p1)
    print(f"{t2-t1:.2}s")
    t1 = time()
    p2 = solve(filename, wrapping_p2)
    print("Part 2:", p2)
    t2 = time()
    print(f"{t2-t1:.2}s")
