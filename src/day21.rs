#![cfg(test)]

use std::collections::VecDeque;

fn is_valid(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && (pos.0 as usize) < grid.len()
        && (pos.1 as usize) < grid[0].len()
        && grid[pos.0 as usize][pos.1 as usize] == '.'
}

#[test]
fn part1() {
    let input = include_str!("../input/day21.txt");
    let start: (i32, i32) = (65, 65);
    let needed = 64;

    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    input[start.0 as usize][start.1 as usize] = '.';

    let mut dists = vec![vec![i32::MAX; input[0].len()]; input.len()];

    let mut q = VecDeque::new();
    q.push_back((start, 0));
    while let Some((pos, dist)) = q.pop_front() {
        if !is_valid(&input, pos) {
            continue;
        }
        if dist >= dists[pos.0 as usize][pos.1 as usize] {
            continue;
        }

        dists[pos.0 as usize][pos.1 as usize] = dist;
        q.push_back(((pos.0 + 1, pos.1), dist + 1));
        q.push_back(((pos.0 - 1, pos.1), dist + 1));
        q.push_back(((pos.0, pos.1 + 1), dist + 1));
        q.push_back(((pos.0, pos.1 - 1), dist + 1));
    }

    let result: usize = dists
        .iter()
        .map(|row| {
            row.iter()
                .filter(|v| **v % 2 == needed % 2 && **v <= needed)
                .count()
        })
        .sum();
    println!("{result}");
}
