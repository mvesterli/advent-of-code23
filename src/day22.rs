#![cfg(test)]

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Block {
    min: (usize, usize, usize),
    max: (usize, usize, usize),
}

fn parse(input: &str) -> Vec<Block> {
    input
        .lines()
        .map(|s| {
            let parts: Vec<usize> = s
                .split(&[',', '~'])
                .filter_map(|v| v.parse().ok())
                .collect();
            Block {
                min: (parts[0], parts[1], parts[2]),
                max: (parts[3], parts[4], parts[5]),
            }
        })
        .collect()
}

fn place(grid: &mut Vec<Vec<Vec<Option<usize>>>>, block: &Block, z: usize, idx: usize) {
    for z in z..=z + block.max.2 - block.min.2 {
        for y in block.min.1..=block.max.1 {
            for x in block.min.0..=block.max.0 {
                grid[z][y][x] = Some(idx);
            }
        }
    }
}

fn get_drop_location(
    grid: &Vec<Vec<Vec<Option<usize>>>>,
    block: &Block,
) -> (HashSet<usize>, usize) {
    let mut supports = HashSet::new();
    for z in (0..block.min.2).rev() {
        for y in block.min.1..=block.max.1 {
            for x in block.min.0..=block.max.0 {
                if let Some(idx) = grid[z][y][x] {
                    supports.insert(idx);
                }
            }
        }
        if !supports.is_empty() {
            return (supports, z + 1);
        }
    }
    (supports, 0)
}

#[test]
fn part1() {
    let input = include_str!("../input/day22.txt");
    let mut blocks = parse(input);
    blocks.sort_by_key(|v| v.min.2);
    let mut can_be_disintegrated = vec![true; blocks.len()];

    let max: usize = 400;
    let mut grid = vec![vec![vec![None; max]; max]; max];

    for (idx, block) in blocks.iter().enumerate() {
        let (supports, z) = get_drop_location(&grid, &block);
        place(&mut grid, &block, z, idx);
        if supports.len() == 1 {
            can_be_disintegrated[*supports.iter().next().unwrap()] = false;
        }
    }
    assert_eq!(can_be_disintegrated.iter().filter(|v| **v).count(), 375);
}

fn count_falls(
    supporting: &Vec<Vec<usize>>,
    num_supports: &Vec<usize>,
    disintegrated: usize,
) -> usize {
    let mut q = VecDeque::new();
    let mut remaining_supports = num_supports.clone();

    for idx in &supporting[disintegrated] {
        q.push_back(*idx);
    }

    let mut res = 0;
    while let Some(i) = q.pop_front() {
        remaining_supports[i] -= 1;
        if remaining_supports[i] == 0 {
            res += 1;
            for idx in &supporting[i] {
                q.push_back(*idx);
            }
        }
    }
    res
}

#[test]
fn part2() {
    let input = include_str!("../input/day22.txt");
    let mut blocks = parse(input);
    blocks.sort_by_key(|v| v.min.2);
    let mut supporting = vec![Vec::new(); blocks.len()];
    let mut num_supports = Vec::new();

    let max: usize = 400;
    let mut grid = vec![vec![vec![None; max]; max]; max];

    for (idx, block) in blocks.iter().enumerate() {
        let (supports, z) = get_drop_location(&grid, &block);
        place(&mut grid, &block, z, idx);
        num_supports.push(supports.len());
        for support in supports {
            supporting[support].push(idx);
        }
    }

    let res: usize = (0..blocks.len())
        .map(|i| count_falls(&supporting, &num_supports, i))
        .sum();
    assert_eq!(res, 72352);
}
