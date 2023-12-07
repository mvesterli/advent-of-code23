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

#[derive(Debug)]
struct Input {
    pub hand: Vec<u8>,
    pub bid: i32,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        Self {
            hand: hand.chars().map(parse_card).collect(),
            bid: bid.parse().unwrap(),
        }
    }
}

impl Input {
    fn get_type(&self) -> u8 {
        let mut card_counts = [0; 15];
        for card in &self.hand {
            card_counts[*card as usize] += 1;
        }
        let mut counts = [0; 6];
        for value in card_counts {
            counts[value] += 1;
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
}

#[test]
fn part1() {
    let input = include_str!("../input/day7.txt");
    let mut input: Vec<_> = input.lines().map(Input::from).collect();
    input.sort_by(|a, b| a.get_type().cmp(&b.get_type()).then(a.hand.cmp(&b.hand)));
    let res: i32 = input
        .iter()
        .enumerate()
        .map(|v| (v.0 + 1) as i32 * v.1.bid)
        .sum();
    println!("{}", res);
}
