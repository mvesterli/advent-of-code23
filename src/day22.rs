#![cfg(test)]

use std::collections::HashSet;

#[derive(Debug)]
struct Block {
    id: usize,
    min: (usize, usize, usize),
    max: (usize, usize, usize),
}

fn parse(input: &str) -> Vec<Block> {
    input
        .lines()
        .enumerate()
        .map(|(id, s)| {
            let parts: Vec<usize> = s
                .split(&[',', '~'])
                .filter_map(|v| v.parse().ok())
                .collect();
            Block {
                id,
                min: (parts[0], parts[1], parts[2]),
                max: (parts[3], parts[4], parts[5]),
            }
        })
        .collect()
}

fn place(grid: &mut Vec<Vec<Vec<Option<usize>>>>, block: &Block, z: usize) {
    for z in z..=z + block.max.2 - block.min.2 {
        for y in block.min.1..=block.max.1 {
            for x in block.min.0..=block.max.0 {
                grid[z][y][x] = Some(block.id);
            }
        }
    }
}

fn write_grid(grid: &Vec<Vec<Vec<Option<usize>>>>) {
    for z in grid {
        for y in z {
            for x in y {
                let c = x
                    .map(|v| char::from_digit(v as u32, 10).unwrap())
                    .unwrap_or('_');
                print!("{}", c);
            }
            println!("");
        }
        println!("");
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

    for block in blocks {
        let (supports, z) = get_drop_location(&grid, &block);
        place(&mut grid, &block, z);
        if supports.len() == 1 {
            can_be_disintegrated[*supports.iter().next().unwrap()] = false;
        }
    }
    println!("{}", can_be_disintegrated.iter().filter(|v| **v).count());
}
