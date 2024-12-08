#![cfg(test)]

fn start_pos(grid: &[Vec<char>]) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                return (y as i32, x as i32);
            }
        }
    }
    unreachable!()
}

fn is_inside(grid: &[Vec<char>], pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < grid.len() as i32 && pos.1 >= 0 && pos.1 < grid[0].len() as i32
}

fn dir_to_flag(dir: (i32, i32)) -> u8 {
    match dir {
        (-1, 0) => 0b1,
        (1, 0) => 0b10,
        (0, 1) => 0b100,
        (0, -1) => 0b1000,
        _ => unreachable!(),
    }
}

fn flatten_coord(grid: &[Vec<char>], pos: (i32, i32)) -> usize {
    (pos.0 * grid[0].len() as i32 + pos.1) as usize
}

fn fill_path(grid: &[Vec<char>], mut pos: (i32, i32), mut dir: (i32, i32)) -> Option<Vec<u8>> {
    let mut visited = vec![0u8; grid.len() * grid[0].len()];

    while (visited[flatten_coord(grid, pos)] & dir_to_flag(dir)) == 0 {
        visited[flatten_coord(grid, pos)] |= dir_to_flag(dir);
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if is_inside(grid, next) {
            if grid[next.0 as usize][next.1 as usize] == '#' {
                dir = (dir.1, -dir.0);
            } else {
                pos = next;
            }
        } else {
            return Some(visited);
        }
    }
    None
}

#[test]
fn part1() {
    let input = include_str!("../input/day6.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let pos = start_pos(&grid);
    let visited = fill_path(&grid, pos, (-1, 0)).unwrap();
    let res: usize = visited.iter().filter(|v| **v != 0).count();
    assert_eq!(5516, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day6.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let pos = start_pos(&grid);
    let dir = (-1, 0);

    let visited = fill_path(&grid, pos, dir).unwrap();

    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' && visited[flatten_coord(&grid, (y as i32, x as i32))] != 0 {
                grid[y][x] = '#';
                if fill_path(&grid, pos, dir).is_none() {
                    res += 1;
                }
                grid[y][x] = '.'
            }
        }
    }
    assert_eq!(2008, res);
}
