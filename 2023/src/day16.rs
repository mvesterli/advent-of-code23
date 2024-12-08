#![cfg(test)]

use std::collections::VecDeque;

fn mov(pos: (i32, i32), dir: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    ((pos.0 + dir.0, pos.1 + dir.1), dir)
}

fn energized(grid: &Vec<Vec<char>>, entry: ((i32, i32), (i32, i32))) -> usize {
    let mut energized = vec![vec![[false; 4]; grid[0].len()]; grid.len()];

    let mut q: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
    q.push_back(entry);
    while let Some((pos, dir)) = q.pop_front() {
        if pos.0 < 0 || pos.0 >= grid.len() as i32 || pos.1 < 0 || pos.1 >= grid[0].len() as i32 {
            continue;
        }

        let dir_idx = match dir {
            (0, 1) => 0,
            (0, -1) => 1,
            (1, 0) => 2,
            (-1, 0) => 3,
            _ => unreachable!(),
        };
        if energized[pos.0 as usize][pos.1 as usize][dir_idx] {
            continue;
        }
        energized[pos.0 as usize][pos.1 as usize][dir_idx] = true;

        match grid[pos.0 as usize][pos.1 as usize] {
            '\\' => q.push_back(mov(pos, (dir.1, dir.0))),
            '/' => q.push_back(mov(pos, (-dir.1, -dir.0))),

            '|' if dir.1 != 0 => {
                q.push_back(mov(pos, (-1, 0)));
                q.push_back(mov(pos, (1, 0)));
            }
            '-' if dir.0 != 0 => {
                q.push_back(mov(pos, (0, -1)));
                q.push_back(mov(pos, (0, 1)));
            }
            _ => q.push_back(mov(pos, dir)),
        }
    }

    energized
        .iter()
        .map(|l| l.iter().filter(|v| v.iter().any(|v| *v)).count())
        .sum()
}

#[test]
fn part1() {
    let input = include_str!("../input/day16.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(energized(&grid, ((0, 0), (0, 1))), 7482);
}

#[test]
fn part2() {
    let input = include_str!("../input/day16.txt");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let res = (0..grid.len() as i32)
        .map(|y| ((y, 0), (0, 1)))
        .chain((0..grid.len() as i32).map(|y| ((y, grid[0].len() as i32 - 1), (0, -1))))
        .chain((0..grid[0].len() as i32).map(|x| ((0, x), (1, 0))))
        .chain((0..grid[0].len() as i32).map(|x| ((grid.len() as i32 - 1, x), (-1, 0))))
        .map(|entry| energized(&grid, entry))
        .max()
        .unwrap();
    assert_eq!(res, 7896);
}
