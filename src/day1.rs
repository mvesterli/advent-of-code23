#![cfg(test)]

use regex::Regex;

#[test]
fn part1() {
    let input = include_str!("../input/day1.txt");

    let mut sum = 0;
    for line in input.lines() {
        let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        sum += first * 10 + last
    }

    println!("{sum}");
}

fn parse(s: &str) -> i32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        s => s.parse().unwrap(),
    }
}

#[test]
fn part2() {
    let input = include_str!("../input/day1.txt");

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut matches = Vec::new();
        let mut offset = 0;
        while let Some(m) = re.find_at(line, offset) {
            matches.push(m.as_str());
            offset = m.start() + 1;
        }
        let first = parse(matches.first().unwrap());
        let last = parse(matches.last().unwrap());

        println!("{line} {matches:?}");

        sum += first * 10 + last
    }

    println!("{sum}");
}
