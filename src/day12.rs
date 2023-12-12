#![cfg(test)]

fn try_place(row: &[char], num: usize) -> Option<&[char]> {
    if row[0..num].iter().any(|v| *v == '.') {
        return None;
    }
    if row.len() == num {
        Some(&[])
    } else {
        if row[num] == '#' {
            None
        } else {
            Some(&row[num + 1..])
        }
    }
}

fn num_options(row: &[char], placed: &[usize]) -> u32 {
    if placed.is_empty() {
        if row.contains(&'#') {
            0
        } else {
            1
        }
    } else if row.len() < placed[0] {
        0
    } else {
        if row[0] == '#' {
            if let Some(rest) = try_place(row, placed[0]) {
                num_options(rest, &placed[1..])
            } else {
                0
            }
        } else if row[0] == '?' {
            let mut res = num_options(&row[1..], placed);
            if let Some(rest) = try_place(row, placed[0]) {
                res += num_options(rest, &placed[1..]);
            }
            res
        } else {
            num_options(&row[1..], placed)
        }
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day12.txt");

    let mut sum = 0;

    for line in input.lines() {
        let (row, check) = line.split_once(' ').unwrap();
        let row: Vec<char> = row.chars().collect();
        let check: Vec<usize> = check.split(',').filter_map(|v| v.parse().ok()).collect();
        sum += num_options(&row, &check);
    }

    println!("{sum}");
}
