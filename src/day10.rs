#![cfg(test)]

fn get_connecting(grid: &Vec<Vec<char>>, (row, col): (i32, i32), prev: (i32, i32)) -> (i32, i32) {
    match (row - prev.0, col - prev.1, grid[row as usize][col as usize]) {
        (1, _, 'L') => (row, col + 1),
        (1, _, '|') => (row + 1, col),
        (1, _, 'J') => (row, col - 1),

        (-1, _, 'F') => (row, col + 1),
        (-1, _, '|') => (row - 1, col),
        (-1, _, '7') => (row, col - 1),

        (_, 1, 'J') => (row - 1, col),
        (_, 1, '-') => (row, col + 1),
        (_, 1, '7') => (row + 1, col),

        (_, -1, 'L') => (row - 1, col),
        (_, -1, '-') => (row, col - 1),
        (_, -1, 'F') => (row + 1, col),

        _ => unreachable!(),
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day10.txt");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = (98, 90);
    let next = (99, 90);

    let mut prev = start;
    let mut pos = next;

    let mut len = 0;
    while pos != start {
        let next = get_connecting(&grid, pos, prev);
        prev = pos;
        pos = next;
        len += 1;
    }
    assert_eq!((len + 1) / 2, 6903);
}

#[test]
fn part2() {
    let input = include_str!("../input/day10.txt");
    let start = (98, 90);
    let next = (99, 90);

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut is_loop = vec![vec![false; grid[0].len()]; grid.len()];
    is_loop[start.0 as usize][start.1 as usize] = true;
    is_loop[next.0 as usize][next.1 as usize] = true;

    let mut prev = start;
    let mut pos = next;

    while pos != start {
        let next = get_connecting(&grid, pos, prev);
        is_loop[next.0 as usize][next.1 as usize] = true;
        prev = pos;
        pos = next;
    }

    let mut res = 0;
    for row in is_loop.iter().zip(&grid) {
        let mut inside = false;
        let mut from_below = false;
        for (is_loop, c) in row.0.iter().zip(row.1) {
            if *is_loop {
                match *c {
                    '|' => inside = !inside,
                    'L' => from_below = false,
                    'F' => from_below = true,
                    'J' => {
                        if from_below {
                            inside = !inside
                        }
                    }
                    '7' => {
                        if !from_below {
                            inside = !inside
                        }
                    }
                    _ => {}
                }
            } else if inside {
                res += 1;
            }
        }
    }
    assert_eq!(res, 265);
}
