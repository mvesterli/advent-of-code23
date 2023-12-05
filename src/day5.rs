#![cfg(test)]

#[allow(unused)]
fn any_source_overlaps(mappings: &Vec<Vec<Vec<i64>>>) -> bool {
    for mapping in mappings {
        for i in 0..mapping.len() {
            let ri = mapping[i][1]..mapping[i][1] + mapping[i][2];
            for j in 0..mapping.len() {
                let rj = mapping[j][1]..mapping[j][1] + mapping[j][2];
                if i != j {
                    if ri.start < rj.end && ri.end > rj.start {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn get_corresponding(mapping: &[Vec<i64>], value: i64) -> i64 {
    for row in mapping {
        let idx = value - row[1];
        if idx >= 0 && idx < row[2] {
            return row[0] + idx;
        }
    }
    value
}

#[test]
fn part1() {
    let input = include_str!("../input/day5.txt");

    let lines: Vec<_> = input.lines().collect();

    let mut mappings = lines.split(|s| s.is_empty());
    let seeds: Vec<i64> = mappings.next().unwrap()[0]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mappings: Vec<Vec<Vec<i64>>> = mappings
        .map(|lines| {
            lines
                .iter()
                .skip(1)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut min = i64::MAX;
    for seed in seeds {
        let mut idx = seed;
        for mapping in &mappings {
            idx = get_corresponding(mapping, idx);
        }
        min = min.min(idx);
    }
    println!("{min}");
}
