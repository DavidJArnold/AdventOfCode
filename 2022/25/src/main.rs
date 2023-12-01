use std::collections::HashMap;

const REAL_FILENAME: &str = "25.real.txt";

fn dec2snafu(num: i64) -> String {
    let mut snafu: Vec<char> = vec![];
    let mut dec = num;
    while dec != 0 {
        let rem = dec % 5;
        if rem < 3 {
            // snafu.push(char::from_u32(rem.try_into().unwrap()).unwrap());
            if rem == 0 {
                snafu.push('0');
            } else if rem == 1 {
                snafu.push('1');
            } else if rem == 2 {
                snafu.push('2')
            } else {
                panic!("Invalid remainder");
            }
            dec = (dec - rem) / 5;
        } else {
            if rem == 4 {
                snafu.push('-');
            } else if rem == 3 {
                snafu.push('=');
            }
            dec = (dec + 5 - rem) / 5;
        }
    }
    snafu.into_iter().rev().collect()
}

fn snafu2dec(snafu: &str) -> i64 {
    let conversions = [('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)];
    let conv: HashMap<char, i64> = conversions.into_iter().collect();
    snafu
        .chars()
        .enumerate()
        .map(|(i, x)| conv.get(&x).unwrap() * 5_i64.pow(snafu.len() as u32 - i as u32 - 1))
        .sum()
}

fn part1(filename: &str) -> String {
    let mut input = std::fs::read_to_string(filename).unwrap();
    input = input.strip_suffix('\n').unwrap().to_string();
    dec2snafu(input.split('\n').map(snafu2dec).sum())
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "25.test.txt";
    use crate::{dec2snafu, part1, snafu2dec};

    #[test]
    fn test_dec2snafu() {
        assert_eq!(dec2snafu(1), "1");
        assert_eq!(dec2snafu(2), "2");
        assert_eq!(dec2snafu(3), "1=");
        assert_eq!(dec2snafu(4), "1-");
        assert_eq!(dec2snafu(5), "10");
        assert_eq!(dec2snafu(6), "11");
        assert_eq!(dec2snafu(7), "12");
        assert_eq!(dec2snafu(8), "2=");
        assert_eq!(dec2snafu(9), "2-");
        assert_eq!(dec2snafu(10), "20");
        assert_eq!(dec2snafu(15), "1=0");
        assert_eq!(dec2snafu(20), "1-0");
        assert_eq!(dec2snafu(2022), "1=11-2");
        assert_eq!(dec2snafu(12345), "1-0---0");
        assert_eq!(dec2snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_snafu2dec() {
        assert_eq!(snafu2dec("1=-0-2"), 1747);
        assert_eq!(snafu2dec("12111"), 906);
        assert_eq!(snafu2dec("2=0="), 198);
        assert_eq!(snafu2dec("21"), 11);
        assert_eq!(snafu2dec("2=01"), 201);
        assert_eq!(snafu2dec("111"), 31);
        assert_eq!(snafu2dec("20012"), 1257);
        assert_eq!(snafu2dec("112"), 32);
        assert_eq!(snafu2dec("1=-1="), 353);
        assert_eq!(snafu2dec("1-12"), 107);
        assert_eq!(snafu2dec("12"), 7);
        assert_eq!(snafu2dec("1="), 3);
        assert_eq!(snafu2dec("122"), 37);
    }

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, "2=-1=0");
    }
}
