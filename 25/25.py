import math
import re


def parse(filename: str) -> list:
    with open(filename, "r", encoding="utf-8") as file:
        input_text = file.read().strip()
    return input_text.split("\n")

def snafu2dec(snafu: str) -> int:
    conv = {'2': 2, '1': 1, '0': 0, '-': -1, '=': -2}
    dec_num = sum(conv[digit]*5**(len(snafu)-idx-1) for idx, digit in enumerate(snafu))
    return dec_num

def dec2snafu(dec: int) -> str:
    snafu = []
    while dec != 0:
        rem = dec % 5
        if rem < 3:
            snafu.append(str(rem))
            dec = (dec - rem)//5
        else:
            if rem == 4:
                snafu.append("-")
            if rem == 3:
                snafu.append("=")
            dec = (dec + 5 - rem)//5
    
    return "".join(reversed(snafu))

def part1(filename: str) -> int:
    data = parse(filename)
    total_dec = sum(snafu2dec(n) for n in data)
    test = dec2snafu(total_dec)
    return test

if __name__ == "__main__":
    FILE_NAME = "25.real.txt"
    p1 = part1(FILE_NAME)
    print("Part 1:", p1)