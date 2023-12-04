def parse_input(filename):
    with open(filename, "r") as file:
        input = file.readlines()

    symbols = []
    nums = []
    for idx, line in enumerate(input):
        next_num = []  # accumulator for numbers
        next_pos_cols = []  # accumulator for columns for the number

        for jdx, char in enumerate(line.strip()):
            if char.isdigit():
                # digits get build into numbers
                next_num.append(char)
                next_pos_cols.append(jdx)
            else:
                # not a digit---first check if we have just gone past the end of a number
                if next_num != []:
                    # make the number and reset accumulators
                    number = int("".join(next_num))
                    nums.append((number, next_pos_cols, idx))
                    next_num = []
                    next_pos_cols = []
                if char != ".":
                    # it's a symbol---record the location and character
                    symbols.append([(jdx, idx), char])
        if next_num != []:  # check if a number finishes at the end of a line
            number = int("".join(next_num))
            nums.append((number, next_pos_cols, idx))
            # accumulators will be reset at the start of the next loop

    return nums, symbols


def part1(filename):
    numbers, symbols = parse_input(filename)
    sym_pos = [y[0] for y in symbols]  # only care about symbol positions

    result = 0
    inserted = False
    for num in numbers:
        # iterate through each number and check the surrounding positions for symbols
        # if the number is adjacent to a symbol, insert it and move to the next number
        for x_off in [-1, 0, 1]:
            for y_off in [-1, 0, 1]:
                if any((x + x_off, num[2] + y_off) in sym_pos for x in num[1]):
                    result += num[0]
                    inserted = True
                    break
            if inserted:
                break
        inserted = False

    return result


def part2(filename):
    numbers, symbols = parse_input(filename)
    # get gear locations
    gear_pos = [y[0] for y in symbols if y[1] == "*"]

    result = 0
    for gear in gear_pos:
        # iterate through gears
        nums = []
        for number in numbers:
            # only test numbers in the same or an adjoing row
            if abs(number[2] - gear[1]) > 1:
                continue
            if gear in (
                (x + x_off, number[2] + y_off)
                for x_off in [-1, 0, 1]
                for y_off in [-1, 0, 1]
                for x in number[1]
            ):
                # number is adjacent, add it to the list
                nums.append(number[0])
            if len(nums) > 2:
                # more than 2 numbers are adjacent---this is not a gear
                break
        if len(nums) == 2:
            # this is a gear
            result += nums[0] * nums[1]
    return result


if __name__ == "__main__":
    filename = "03.real.txt"
    p1 = part1(filename)
    print(f"Part 1: {p1}")
    p2 = part2(filename)
    print(f"Part 2: {p2}")
