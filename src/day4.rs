#![cfg(test)]

use std::collections::HashSet;

#[test]
fn part1() {
    let input = include_str!("../input/day4.txt");

    let mut sum = 0;

    for card in input.lines() {
        let (_, contents) = card.split_once(':').unwrap();
        let (winning, owned) = contents.split_once('|').unwrap();

        let winning: HashSet<i32> = winning
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let mut points = 0;
        let owned = owned
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>().unwrap());
        for card in owned {
            if winning.contains(&card) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }
        sum += points;
    }

    assert_eq!(sum, 26914);
}

#[test]
fn part2() {
    let input = include_str!("../input/day4.txt");

    let mut copies: Vec<_> = input.lines().map(|_| 1).collect();

    for (idx, card) in input.lines().enumerate() {
        let (_, contents) = card.split_once(':').unwrap();
        let (winning, owned) = contents.split_once('|').unwrap();

        let winning: HashSet<i32> = winning
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let matches = owned
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .filter(|c| winning.contains(c))
            .count();
        for i in idx + 1..=idx + matches {
            if i < copies.len() {
                copies[i] += copies[idx];
            }
        }
    }

    assert_eq!(copies.iter().sum::<i32>(), 13080971);
}
