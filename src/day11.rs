#![cfg(test)]

fn get_positions(input: &str) -> Vec<(i64, i64)> {
    let mut positions = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push((row as i64, col as i64));
            }
        }
    }
    positions
}

fn get_distances(positions: Vec<(i64, i64)>, expansion: i64) -> i64 {
    let (rows, cols): (Vec<_>, Vec<_>) = positions.into_iter().unzip();
    let mut res = 0;
    for row in &mut [rows, cols] {
        row.sort();
        let mut dists = 0;
        for (idx, w) in row.windows(2).enumerate() {
            dists += (idx as i64 + 1) * ((w[1] - w[0] - 1) * expansion + 1).max(0);
            res += dists;
        }
    }
    res
}

#[test]
fn part1() {
    let input = include_str!("../input/day11.txt");
    println!("{}", get_distances(get_positions(input), 2));
}

#[test]
fn part2() {
    let input = include_str!("../input/day11.txt");
    println!("{}", get_distances(get_positions(input), 1_000_000));
}
