#![cfg(test)]

use regex::Regex;

fn get_positions(input: &Vec<(char, i64)>) -> Vec<(i64, i64)> {
    input
        .iter()
        .scan((0, 0), |pos, (dir, count)| {
            *pos = match dir {
                'R' => (pos.0, pos.1 + count),
                'D' => (pos.0 + count, pos.1),
                'L' => (pos.0, pos.1 - count),
                'U' => (pos.0 - count, pos.1),
                _ => unreachable!(),
            };
            Some(*pos)
        })
        .collect()
}

fn count_filled(positions: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    let mut len = 0;
    for i in 0..positions.len() {
        let next = (i + 1) % positions.len();
        sum += positions[i].0 * positions[next].1 - positions[i].1 * positions[next].0;
        len +=
            (positions[i].0 - positions[next].0).abs() + (positions[i].1 - positions[next].1).abs();
    }
    (sum.abs() + len) / 2 + 1
}

fn parse1(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| {
            let (dir, rest) = line.split_once(' ').unwrap();
            let num = rest.split_once(' ').unwrap().0.parse().unwrap();
            (dir.chars().next().unwrap(), num)
        })
        .collect()
}

#[test]
fn part1() {
    let input = include_str!("../input/day18.txt");
    let input = parse1(input);
    let positions = get_positions(&input);
    assert_eq!(count_filled(&positions), 26857);
}

fn convert_instruction(c: char) -> char {
    match c {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => unreachable!(),
    }
}

fn parse2(input: &str) -> Vec<(char, i64)> {
    let re = Regex::new(r"\w \d+ \(#(\w+)\)").unwrap();
    input
        .lines()
        .map(|l| re.captures(l).unwrap().extract::<1>().1[0])
        .map(|s| {
            (
                convert_instruction(s.chars().nth(5).unwrap()),
                i64::from_str_radix(&s[0..5], 16).unwrap(),
            )
        })
        .collect()
}

#[test]
fn part2() {
    let input = include_str!("../input/day18.txt");
    let input = parse2(input);
    let positions = get_positions(&input);
    assert_eq!(count_filled(&positions), 129373230496292);
}
