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

fn get_score_1((rows, cols): (Vec<u32>, Vec<u32>)) -> usize {
    for i in 1..rows.len() {
        let is_mirror = (0..i)
            .rev()
            .zip(i..rows.len())
            .all(|(a, b)| rows[a] == rows[b]);
        if is_mirror {
            return 100 * i;
        }
    }
    for i in 1..cols.len() {
        let is_mirror = (0..i)
            .rev()
            .zip(i..cols.len())
            .all(|(a, b)| cols[a] == cols[b]);
        if is_mirror {
            return i;
        }
    }
    0
}

fn get_score_2((rows, cols): (Vec<u32>, Vec<u32>)) -> usize {
    for i in 1..rows.len() {
        let is_mirror = (0..i)
            .rev()
            .zip(i..rows.len())
            .map(|(a, b)| (rows[a] ^ rows[b]).count_ones())
            .sum::<u32>()
            == 1;
        if is_mirror {
            return 100 * i;
        }
    }
    for i in 1..cols.len() {
        let is_mirror = (0..i)
            .rev()
            .zip(i..cols.len())
            .map(|(a, b)| (cols[a] ^ cols[b]).count_ones())
            .sum::<u32>()
            == 1;
        if is_mirror {
            return i;
        }
    }
    0
}

#[test]
fn part1() {
    let input = include_str!("../input/day13.txt");
    let lines: Vec<_> = input.lines().collect();
    let patterns = lines.split(|s| s.is_empty());
    let sum: usize = patterns.map(parse_bin).map(get_score_1).sum();
    println!("{sum}");
}

#[test]
fn part2() {
    let input = include_str!("../input/day13.txt");
    let lines: Vec<_> = input.lines().collect();
    let patterns = lines.split(|s| s.is_empty());
    let sum: usize = patterns.map(parse_bin).map(get_score_2).sum();
    println!("{sum}");
}
