#![cfg(test)]

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(':');
    let result: u64 = parts.next().unwrap().parse().unwrap();
    let numbers: Vec<u64> = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    (result, numbers)
}

fn is_solvable(value: u64, numbers: &[u64], include_cons: bool) -> bool {
    if numbers.len() == 1 {
        return value == numbers[0];
    }
    let last = *numbers.last().unwrap();
    let rest = &numbers[..numbers.len() - 1];

    let mut res = false;
    if last <= value {
        res |= is_solvable(value - last, rest, include_cons);
    }
    if value % last == 0 {
        res |= is_solvable(value / last, rest, include_cons);
    }
    let div = 10u64.pow(last.ilog10() + 1);
    if include_cons && value % div == last {
        res |= is_solvable(value / div, rest, include_cons);
    }
    res
}

#[test]
fn part1() {
    let input = include_str!("../input/day7.txt");
    let result: u64 = input
        .lines()
        .map(parse_line)
        .filter(|(res, nums)| is_solvable(*res, nums, false))
        .map(|p| p.0)
        .sum();
    assert_eq!(1611660863222, result);
}

#[test]
fn part2() {
    let input = include_str!("../input/day7.txt");
    let result: u64 = input
        .lines()
        .map(parse_line)
        .filter(|(res, nums)| is_solvable(*res, nums, true))
        .map(|p| p.0)
        .sum();
    assert_eq!(945341732469724, result);
}
