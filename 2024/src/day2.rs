#![cfg(test)]

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(v: &[i32]) -> bool {
    v.windows(2).all(|v| v[1] - v[0] >= 1 && v[1] - v[0] <= 3)
        || v.windows(2).all(|v| v[0] - v[1] >= 1 && v[0] - v[1] <= 3)
}

fn is_safe_tol(v: &[i32]) -> bool {
    if is_safe(v) {
        return true;
    }
    for i in 0..v.len() {
        let mut copy = v.to_owned();
        copy.remove(i);
        if is_safe(&copy) {
            return true;
        }
    }
    false
}

#[test]
fn part1() {
    let input = include_str!("../input/day2.txt");
    let reports = parse(input);
    let result = reports.iter().filter(|v| is_safe(v)).count();
    assert_eq!(321, result);
}

#[test]
fn part2() {
    let input = include_str!("../input/day2.txt");
    let reports = parse(input);
    let result = reports.iter().filter(|v| is_safe_tol(v)).count();
    assert_eq!(386, result);
}
