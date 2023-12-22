#![cfg(test)]

use std::collections::VecDeque;

fn is_valid(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && (pos.0 as usize) < grid.len()
        && (pos.1 as usize) < grid[0].len()
        && grid[pos.0 as usize][pos.1 as usize] != '#'
}

fn dists_from(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> Vec<Vec<i64>> {
    let mut dists = vec![vec![i64::MAX; grid[0].len()]; grid.len()];

    let mut q = VecDeque::new();
    q.push_back((pos, 0));
    while let Some((pos, dist)) = q.pop_front() {
        if !is_valid(&grid, pos) {
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
    dists
}

fn count_le_than(dists: &Vec<Vec<i64>>, val: i64) -> usize {
    dists
        .iter()
        .map(|row| {
            row.iter()
                .filter(|v| **v % 2 == val % 2 && **v <= val)
                .count()
        })
        .sum::<usize>()
}

#[test]
fn part1() {
    let input = include_str!("../input/day21.txt");
    let start = (65, 65);
    let needed = 64;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let dists = dists_from(&grid, start);
    println!("{}", count_le_than(&dists, needed));
}

#[test]
fn part2() {
    let input = include_str!("../input/day21.txt");
    let start = (65, 65);
    let needed = 26501365;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let max_idx = grid.len() as i32 - 1;
    let corner_dists = [
        dists_from(&grid, (0, 0)),
        dists_from(&grid, (0, max_idx)),
        dists_from(&grid, (max_idx, max_idx)),
        dists_from(&grid, (max_idx, 0)),
    ];

    let side_dists = [
        dists_from(&grid, (0, start.1)),
        dists_from(&grid, (max_idx, start.1)),
        dists_from(&grid, (start.0, 0)),
        dists_from(&grid, (start.0, max_idx)),
    ];

    let reachable_even = count_le_than(&corner_dists[0], 100000) as i64;
    let reachable_odd = count_le_than(&corner_dists[0], 100001) as i64;

    let needed_from_corner = (needed - start.0 - start.1) as i64;
    let cells = needed_from_corner / grid.len() as i64;
    let rem_far = needed_from_corner % grid.len() as i64;
    let rem_close = rem_far + grid.len() as i64;

    let mut result = 0;
    result += cells * cells * reachable_odd;
    result += ((cells + 1) * (cells + 1)) * reachable_even;
    for corner in &corner_dists {
        result += cells * count_le_than(corner, rem_close - 2) as i64;
        result += (cells + 1) * count_le_than(corner, rem_far - 2) as i64;
    }
    for side in &side_dists {
        result += count_le_than(side, rem_far - 1 + start.0 as i64) as i64;
    }
    println!("{result}");
}
