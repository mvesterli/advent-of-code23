#![cfg(test)]
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    (left, right)
}

#[test]
fn part1() {
    let input = include_str!("../input/day1.txt");
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    let result: i32 = (left.iter().zip(right)).map(|(a, b)| (a - b).abs()).sum();
    assert_eq!(1388114, result);
}

#[test]
fn part2() {
    let input = include_str!("../input/day1.txt");
    let (left, right) = parse(input);

    let mut counts = HashMap::new();
    for v in right {
        *counts.entry(v).or_insert(0) += 1;
    }
    let result: i32 = left
        .iter()
        .map(|v| v * counts.get(v).copied().unwrap_or(0))
        .sum();
    assert_eq!(23529853, result);
}
