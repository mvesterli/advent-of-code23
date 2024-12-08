#![cfg(test)]

fn is_xmas(grid: &[Vec<char>], pos: (i32, i32), dir: (i32, i32)) -> bool {
    for (idx, c) in "XMAS".chars().enumerate() {
        let p = (pos.0 + idx as i32 * dir.0, pos.1 + idx as i32 * dir.1);
        if p.0 >= grid.len() as i32 || p.0 < 0 || p.1 >= grid[0].len() as i32 || p.1 < 0 {
            return false;
        }
        if grid[p.0 as usize][p.1 as usize] != c {
            return false;
        }
    }
    true
}

#[test]
fn part1() {
    let input = include_str!("../input/day4.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for dir in dirs {
                if is_xmas(&grid, (y as i32, x as i32), dir) {
                    res += 1;
                }
            }
        }
    }
    assert_eq!(2557, res);
}

#[test]
fn par2() {
    let input = include_str!("../input/day4.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut res = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] == 'A' {
                let corners = String::from_iter([
                    grid[y - 1][x - 1],
                    grid[y - 1][x + 1],
                    grid[y + 1][x + 1],
                    grid[y + 1][x - 1],
                ]);
                match corners.as_str() {
                    "MMSS" | "MSSM" | "SSMM" | "SMMS" => res += 1,
                    _ => {}
                }
            }
        }
    }
    assert_eq!(1854, res);
}
