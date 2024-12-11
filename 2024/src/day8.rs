#![cfg(test)]

fn get_positions(grid: &[Vec<char>], c: char) -> Vec<(i32, i32)> {
    grid.iter()
        .enumerate()
        .flat_map(|r| {
            r.1.iter()
                .enumerate()
                .filter(|v| *v.1 == c)
                .map(move |v| (r.0 as i32, v.0 as i32))
        })
        .collect()
}

fn is_inside(grid: &[Vec<char>], p: (i32, i32)) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as i32 && p.1 < grid[0].len() as i32
}

#[test]
fn part1() {
    let input = include_str!("../input/day8.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut anti_nodes = vec![vec![false; grid[0].len()]; grid.len()];
    for c in ('a'..='z').chain('0'..='9').chain('A'..='Z') {
        let positions = get_positions(&grid, c);
        for (i, a) in positions.iter().enumerate() {
            for b in &positions[i + 1..] {
                let d = (a.0 - b.0, a.1 - b.1);
                for p in [(a.0 + d.0, a.1 + d.1), (b.0 - d.0, b.1 - d.1)] {
                    if is_inside(&grid, p) {
                        anti_nodes[p.0 as usize][p.1 as usize] = true;
                    }
                }
            }
        }
    }
    let res: usize = anti_nodes
        .iter()
        .map(|v| v.iter().filter(|v| **v).count())
        .sum();
    assert_eq!(361, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day8.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut anti_nodes = vec![vec![false; grid[0].len()]; grid.len()];
    for c in ('a'..='z').chain('0'..='9').chain('A'..='Z') {
        let positions = get_positions(&grid, c);
        for (i, a) in positions.iter().enumerate() {
            for b in &positions[i + 1..] {
                let d = (a.0 - b.0, a.1 - b.1);

                for sign in [1, -1] {
                    let mut p = *a;
                    while is_inside(&grid, p) {
                        anti_nodes[p.0 as usize][p.1 as usize] = true;
                        p.0 += sign * d.0;
                        p.1 += sign * d.1;
                    }
                }
            }
        }
    }
    let res: usize = anti_nodes
        .iter()
        .map(|v| v.iter().filter(|v| **v).count())
        .sum();
    assert_eq!(1249, res);
}
