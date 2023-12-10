#![cfg(test)]

fn create_rows(input: &str) -> Vec<Vec<i32>> {
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut rows = vec![input];
    while rows.last().unwrap().iter().any(|v| *v != 0) {
        rows.push(
            rows.last()
                .unwrap()
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect(),
        );
    }
    rows
}

#[test]
fn part1() {
    let input = include_str!("../input/day9.txt");
    let res = input
        .lines()
        .map(create_rows)
        .map(|rows| rows.iter().map(|r| r.last().unwrap()).sum::<i32>())
        .sum::<i32>();
    println!("{}", res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day9.txt");
    let res = input
        .lines()
        .map(create_rows)
        .map(|rows| {
            rows.iter()
                .enumerate()
                .map(|(idx, r)| -(idx as i32 % 2 * 2 - 1) * r[0])
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{}", res);
}
