#![cfg(test)]

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col_idx in (0..grid[0].len()) {
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
    println!("{}", load(&input));
}
