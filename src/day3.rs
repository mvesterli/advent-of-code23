#![cfg(test)]

use regex::{Match, Regex};

fn is_part_num(chars: &Vec<Vec<char>>, line: usize, num: &Match) -> bool {
    let y_start = line.saturating_sub(1);
    let y_end = (line + 1).min(chars.len() - 1);
    let x_start = num.start().saturating_sub(1);
    let x_end = num.end().min(chars[line].len() - 1);
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let c = chars[y][x];
            if c != '.' && !c.is_numeric() {
                return true;
            }
        }
    }
    false
}

#[test]
fn part1() {
    let input = include_str!("../input/day3.txt");

    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<Vec<_>> = input.lines().map(|l| re.find_iter(l).collect()).collect();
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for (idx, line) in nums.iter().enumerate() {
        for num in line {
            if is_part_num(&chars, idx, num) {
                sum += num.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("{sum}");
}

#[test]
fn part2() {
    let input = include_str!("../input/day3.txt");

    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<Vec<_>> = input.lines().map(|l| re.find_iter(l).collect()).collect();
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for (row, line) in chars.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '*' {
                let mut parts: Vec<i64> = Vec::new();
                for row in row.saturating_sub(1)..=row + 1 {
                    for num in &nums[row] {
                        if num.start() <= col + 1 && num.end() >= col {
                            parts.push(num.as_str().parse().unwrap());
                        }
                    }
                }
                if parts.len() == 2 {
                    sum += parts[0] * parts[1];
                }
            }
        }
    }

    println!("{sum}");
}
