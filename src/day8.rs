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

#[test]
fn part2() {
    let input = include_str!("../input/day8.txt");

    let mut lines = input.lines();
    let steps: Vec<_> = lines.next().unwrap().chars().collect();
    lines.next();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let input: Vec<[&str; 3]> = lines
        .map(|line| re.captures(line).unwrap().extract().1)
        .collect();

    let mut node_to_id = HashMap::new();
    for line in &input {
        node_to_id.insert(line[0], node_to_id.len());
    }

    let mut neighbors = vec![(0, 0); node_to_id.len()];
    for line in &input {
        neighbors[node_to_id[line[0]]] = (node_to_id[line[1]], node_to_id[line[2]]);
    }

    let starts: Vec<_> = input
        .iter()
        .enumerate()
        .filter(|v| v.1[0].ends_with('A'))
        .map(|v| v.0)
        .collect();

    let is_end: Vec<_> = input.iter().map(|v| v[0].ends_with('Z')).collect();

    let mut results = Vec::new();
    for start in starts {
        let mut node = start;
        let mut step = 0;
        let mut dist = 0;
        while !is_end[node] {
            node = if steps[step] == 'L' {
                neighbors[node].0
            } else {
                neighbors[node].1
            };
            step = (step + 1) % steps.len();
            dist += 1;
        }
        // Apparently it was not necessary to handle looping around to
        // multiple end nodes or the same one multiple times
        results.push(dist);
    }

    let res = results.into_iter().fold(1i64, num::integer::lcm);
    println!("{res}");
}
