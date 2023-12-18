#![cfg(test)]

use std::cell::Cell;

fn traverse(input: &str, mut f: impl FnMut((i32, i32))) {
    let mut pos = (0, 0);
    f(pos);
    for line in input.lines() {
        let (dir, rem) = line.split_once(' ').unwrap();
        let (num, color) = rem.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        for _ in 0..num {
            match dir {
                "D" => pos.0 += 1,
                "U" => pos.0 -= 1,
                "R" => pos.1 += 1,
                "L" => pos.1 -= 1,
                _ => {}
            }
            f(pos);
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CellPosition {
    Inside,
    Outside,
    Border,
    Unknown,
}

fn flood_fill(grid: &mut Vec<Vec<CellPosition>>, state: CellPosition, start: (i32, i32)) {
    let mut q = Vec::new();
    q.push(start);
    while let Some(pos) = q.pop() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as i32 || pos.1 >= grid[0].len() as i32 {
            continue;
        }
        if grid[pos.0 as usize][pos.1 as usize] != CellPosition::Unknown {
            continue;
        }
        grid[pos.0 as usize][pos.1 as usize] = state;
        q.push((pos.0 + 1, pos.1));
        q.push((pos.0 - 1, pos.1));
        q.push((pos.0, pos.1 + 1));
        q.push((pos.0, pos.1 - 1));
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day18.txt");

    let mut min = (0, 0);
    let mut max = (0, 0);
    traverse(input, |pos| {
        min.0 = min.0.min(pos.0);
        min.1 = min.1.min(pos.1);
        max.0 = max.0.max(pos.0);
        max.1 = max.1.max(pos.1);
    });

    let offset = (min.0.abs() + 1, min.1.abs() + 1);
    let size = (max.0 + 3 + offset.0, max.1 + 3 + offset.1);
    let mut grid = vec![vec![CellPosition::Unknown; size.1 as usize]; size.0 as usize];

    traverse(input, |pos| {
        grid[(pos.0 + offset.0) as usize][(pos.1 + offset.1) as usize] = CellPosition::Border;
    });
    flood_fill(&mut grid, CellPosition::Outside, (0, 0));

    let result: usize = grid
        .iter()
        .map(|l| l.iter().filter(|v| **v != CellPosition::Outside).count())
        .sum();
    println!("{}", result);
}
