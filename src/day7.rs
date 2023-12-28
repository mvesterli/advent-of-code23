#![cfg(test)]

fn parse_card(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        num => num.to_digit(10).unwrap() as u8,
    }
}

fn parse_card2(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        num => num.to_digit(10).unwrap() as u8,
    }
}

fn type_from_count(card_counts: &[usize; 15]) -> u8 {
    let mut counts = [0; 6];
    for value in &card_counts[2..] {
        counts[*value] += 1;
    }
    if counts[5] != 0 {
        7
    } else if counts[4] != 0 {
        6
    } else if counts[3] != 0 && counts[2] != 0 {
        5
    } else if counts[3] != 0 {
        4
    } else if counts[2] == 2 {
        3
    } else if counts[2] != 0 {
        2
    } else {
        1
    }
}

#[derive(Debug)]
struct Input {
    pub hand: Vec<u8>,
    pub bid: i32,
}

impl Input {
    fn read(value: &str, f: fn(char) -> u8) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        Self {
            hand: hand.chars().map(f).collect(),
            bid: bid.parse().unwrap(),
        }
    }

    fn get_type(&self) -> u8 {
        let mut card_counts = [0; 15];
        for card in &self.hand {
            card_counts[*card as usize] += 1;
        }
        type_from_count(&card_counts)
    }

    fn get_type2(&self) -> u8 {
        let mut card_counts = [0; 15];
        for card in &self.hand {
            card_counts[*card as usize] += 1;
        }
        if card_counts[1] == 5 {
            return 7;
        }
        *card_counts[2..].iter_mut().max().unwrap() += card_counts[1];
        type_from_count(&card_counts)
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day7.txt");
    let mut input: Vec<_> = input.lines().map(|v| Input::read(v, parse_card)).collect();
    input.sort_by(|a, b| a.get_type().cmp(&b.get_type()).then(a.hand.cmp(&b.hand)));
    let res: i32 = input
        .iter()
        .enumerate()
        .map(|v| (v.0 + 1) as i32 * v.1.bid)
        .sum();
    assert_eq!(res, 250120186);
}

#[test]
fn part2() {
    let input = include_str!("../input/day7.txt");
    let mut input: Vec<_> = input.lines().map(|v| Input::read(v, parse_card2)).collect();

    input.sort_by(|a, b| a.get_type2().cmp(&b.get_type2()).then(a.hand.cmp(&b.hand)));
    let res: i32 = input
        .iter()
        .enumerate()
        .map(|v| (v.0 + 1) as i32 * v.1.bid)
        .sum();
    assert_eq!(res, 250665248);
}
