#![cfg(test)]

use std::collections::HashMap;

use regex::Regex;

#[test]
fn part1() {
    let input = include_str!("../input/day8.txt");

    let mut lines = input.lines();
    let mut steps = lines.next().unwrap().chars().cycle();
    lines.next();

    let mut network = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines {
        let parts: [&str; 3] = re.captures(line).unwrap().extract().1;
        network.insert(parts[0], (parts[1], parts[2]));
    }

    let mut res = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        let neighbors = network[pos];
        pos = if steps.next().unwrap() == 'L' {
            neighbors.0
        } else {
            neighbors.1
        };
        res += 1;
    }
    println!("{}", res);
}
