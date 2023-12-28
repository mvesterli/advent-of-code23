#![cfg(test)]

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col_idx in 0..grid[0].len() {
        let mut next_empty = 0;
        for row_idx in 0..grid.len() {
            match grid[row_idx][col_idx] {
                'O' => {
                    grid[row_idx][col_idx] = '.';
                    grid[next_empty][col_idx] = 'O';
                    next_empty += 1;
                }
                '#' => {
                    next_empty = row_idx + 1;
                }
                _ => {}
            }
        }
    }
}

fn turn_right(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = grid.clone();
    for row in 0..res.len() {
        for col in 0..res[0].len() {
            res[row][col] = grid[res.len() - 1 - col][row];
        }
    }
    res
}

fn cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = grid.clone();
    for _ in 0..4 {
        tilt_north(&mut res);
        res = turn_right(res);
    }
    res
}

fn load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|c| **c == 'O').count() * (grid.len() - idx))
        .sum()
}

#[test]
fn part1() {
    let input = include_str!("../input/day14.txt");
    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    tilt_north(&mut input);
    assert_eq!(load(&input), 105784);
}

fn solve2(input: &str) -> usize {
    let mut input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut results = Vec::new();
    for current_cycle in 0.. {
        let next = cycle(&input);
        for (idx, grid) in results.iter().enumerate() {
            if next == *grid {
                let loopback_len = current_cycle - idx + 1;
                if (1000_000_000 - current_cycle) % loopback_len == 0 {
                    return load(&input);
                }
            }
        }
        results.push(input);
        input = next;
    }
    0
}

#[test]
fn part2() {
    let input = include_str!("../input/day14.txt");
    assert_eq!(solve2(input), 91286);
}
