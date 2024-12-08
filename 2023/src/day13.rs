#![cfg(test)]

fn parse_bin(pattern: &[&str]) -> (Vec<u32>, Vec<u32>) {
    let rows: Vec<u32> = pattern
        .iter()
        .map(|line| {
            line.chars()
                .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
        })
        .collect();
    let cols: Vec<u32> = (0..pattern[0].len())
        .rev()
        .map(|idx| {
            rows.iter().fold(0, |acc, v| {
                (acc << 1) | if (v & (1 << idx)) == 0 { 0 } else { 1 }
            })
        })
        .collect();
    (rows, cols)
}

fn is_mirror(rows: &[u32], idx: usize, smudges: u32) -> bool {
    (0..idx)
        .rev()
        .zip(idx..rows.len())
        .map(|(a, b)| (rows[a] ^ rows[b]).count_ones())
        .sum::<u32>()
        == smudges
}

fn get_score(rows: Vec<u32>, cols: Vec<u32>, smudges: u32) -> usize {
    for i in 1..rows.len() {
        if is_mirror(&rows, i, smudges) {
            return 100 * i;
        }
    }
    for i in 1..cols.len() {
        if is_mirror(&cols, i, smudges) {
            return i;
        }
    }
    0
}

fn solve(input: &str, smudges: u32) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let patterns = lines.split(|s| s.is_empty());
    patterns
        .map(parse_bin)
        .map(|(rows, cols)| get_score(rows, cols, smudges))
        .sum::<usize>()
}

#[test]
fn part1() {
    let input = include_str!("../input/day13.txt");
    assert_eq!(solve(input, 0), 30705);
}

#[test]
fn part2() {
    let input = include_str!("../input/day13.txt");
    assert_eq!(solve(input, 1), 44615);
}
