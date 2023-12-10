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
    println!("{}", (len + 1) / 2);
}
