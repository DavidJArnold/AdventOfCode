use core::cmp::Ordering;
use std::collections::HashMap;

const REAL_FILENAME: &str = "07.real.txt";

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
}

fn score_hand(hand: &Vec<u32>) -> u8 {
    let mut hand_map: HashMap<u32, usize> = HashMap::new();
    let mut num_jokers = 0;
    for card in hand {
        if card != &1 {
            *hand_map.entry(*card).or_insert(0) += 1;
        } else {
            num_jokers += 1;
        }
    }
    let mut hand_set: Vec<_> = hand_map.values().collect();
    if hand_set.is_empty() {
        hand_set.push(&0);
    }
    hand_set.sort();
    hand_set.reverse();
    let new_val = *hand_set[0] + num_jokers;
    hand_set[0] = &new_val;
    if hand_set[0] == &5 {
        6
    } else if hand_set[0] == &4 {
        5
    } else if hand_set[0] == &3 && hand_set[1] == &2 {
        4
    } else if hand_set[0] == &3 {
        3
    } else if hand_set[0] == &2 && hand_set[1] == &2 {
        2
    } else if hand_set[0] == &2 {
        1
    } else {
        0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = score_hand(&self.cards);
        let other_score = score_hand(&other.cards);
        match self_score.cmp(&other_score) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for idx in 0..5 {
                    match self.cards[idx].cmp(&other.cards[idx]) {
                        Ordering::Greater => { return Ordering::Greater},
                        Ordering::Less => {return  Ordering::Less},
                        _ => ()
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let mut my_cards = self.cards.clone();
        my_cards.sort();
        let mut other_cards = other.cards.clone();
        other_cards.sort();
        my_cards.eq(&other_cards)
    }
}

impl Eq for Hand {}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    let card_map: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('1', 1),
    ]);

    let mut outputs: Vec<(Hand, u32)> = vec![];
    for line in input.strip_suffix('\n').unwrap_or(input).split('\n') {
        let mut hand: Vec<u32> = vec![];
        let mut line_contents = line.split_whitespace();
        for card in line_contents.next().unwrap().chars() {
            hand.push(*card_map.get(&card).unwrap());
        }
        outputs.push((
            Hand { cards: hand },
            line_contents.next().unwrap().parse::<u32>().unwrap(),
        ));
    }
    outputs
}

fn part1(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let input = file_contents.strip_suffix('\n').unwrap();
    let mut data = parse_input(input);
    data.sort_by_key(|x| Hand {
        cards: x.0.cards.clone(),
    });
    data.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as u32 + 1_u32) * x.1)
}

fn part2(filename: &str) -> u32 {
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let new_contents = file_contents.replace('J', "1");
    let input = new_contents.strip_suffix('\n').unwrap();
    let mut data = parse_input(input);
    data.sort_by_key(|x| Hand {
        cards: x.0.cards.clone(),
    });
    data.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i as u32 + 1_u32) * x.1)
}

fn main() {
    let filename = REAL_FILENAME;
    let ans1 = part1(filename);
    println!("Part 1: {}", ans1);
    let ans2 = part2(filename);
    println!("Part 2: {}", ans2);
}

#[cfg(test)]
mod tests {
    const TEST_FILENAME: &str = "07.test.txt";
    use crate::*;

    #[test]
    fn test_part1() {
        let ans1 = part1(&TEST_FILENAME);
        assert_eq!(ans1, 6440)
    }

    #[test]
    fn test_part2() {
        let ans2 = part2(&TEST_FILENAME);
        assert_eq!(ans2, 5905)
    }

    #[test]
    fn test_parsing() {
        let file_contents = std::fs::read_to_string(TEST_FILENAME).unwrap();
        let input = file_contents.strip_suffix('\n').unwrap();
        let data = parse_input(input);
        assert_eq!(
            data[0],
            (
                Hand {
                    cards: vec![3, 2, 10, 3, 13]
                },
                765
            )
        );
    }

    #[test]
    fn test_hand_cmp() {
        let hand1 = Hand {
            cards: vec![2, 2, 2, 2, 2],
        };
        let hand2 = Hand {
            cards: vec![1, 1, 1, 1, 1],
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
        let hand3 = Hand {
            cards: vec![3, 4, 5, 6, 6],
        };
        let hand4 = Hand {
            cards: vec![1, 5, 4, 3, 2],
        };
        assert_eq!(hand3.cmp(&hand4), Ordering::Greater);
    }
}
