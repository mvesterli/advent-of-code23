#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day9.txt");

    let mut res = 0;
    for line in input.lines() {
        let input: Vec<i32> = line
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
        res += rows.iter().map(|r| r.last().unwrap()).sum::<i32>();
    }
    println!("{res}");
}
