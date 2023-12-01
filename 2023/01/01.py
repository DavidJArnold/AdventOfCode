def parse_input(filename):
    with open(filename, 'r') as file:
        input = file.readlines()
    return input


def calculate_calibration(parsed_numbers):
    vals = []
    for line in parsed_numbers:
        digits = []
        for char in line:
            if char in [str(x) for x in range(1, 10)]:
                digits.append(int(char))
        vals.append(10 * digits[0] + digits[-1])
    return sum(vals)


def parse_words(line):
    translate = {
            "one": 1,
            "two": 2,
            "three": 3,
            "four": 4,
            "five": 5,
            "six": 6,
            "seven": 7,
            "eight": 8,
            "nine": 9,
            }
    newline = ""
    for idx in range(len(line)):
        if line[idx] in [str(x) for x in translate.values()]:
            newline += line[idx]
            continue
        for jdx in range(3, 6):
            if idx + jdx <= len(line):
                if line[idx:idx+jdx] in translate.keys():
                    newline += str(translate[line[idx:idx+jdx]])
    return newline


def part1(filename):
    input = parse_input(filename)
    return calculate_calibration(input)


def part2(filename):
    text_input = parse_input(filename)
    parsed_numbers = [parse_words(line.strip("\n")) for line in text_input]
    return calculate_calibration(parsed_numbers)


if __name__ == "__main__":
    filename = "01.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
