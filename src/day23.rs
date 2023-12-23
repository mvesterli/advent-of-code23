#![cfg(test)]

fn empty_idx(line: &[char]) -> usize {
    line.iter().enumerate().find(|v| *v.1 == '.').unwrap().0
}

fn allowed_directions(cell: char) -> &'static [(i32, i32)] {
    match cell {
        '.' => &[(-1, 0), (0, 1), (1, 0), (0, -1)],
        '^' => &[(-1, 0)],
        'v' => &[(1, 0)],
        '<' => &[(0, -1)],
        '>' => &[(0, 1)],
        _ => unreachable!(),
    }
}

fn get_longest_path(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    node: (i32, i32),
    end: (i32, i32),
) -> Option<usize> {
    if node == end {
        return Some(0);
    }
    if node.0 < 0 || node.1 < 0 || node.0 >= grid.len() as i32 || node.1 >= grid[0].len() as i32 {
        return None;
    }
    let unode = (node.0 as usize, node.1 as usize);
    if grid[unode.0][unode.1] == '#' || visited[unode.0][unode.1] {
        return None;
    }

    visited[unode.0][unode.1] = true;
    let mut res = None;
    for m in allowed_directions(grid[unode.0][unode.1]) {
        let option = get_longest_path(grid, visited, (node.0 + m.0, node.1 + m.1), end);
        if res.is_none() || (option.is_some() && option.unwrap() > res.unwrap()) {
            res = option;
        }
    }
    visited[unode.0][unode.1] = false;
    res.map(|v| v + 1)
}

#[test]
fn part1() {
    let input = include_str!("../input/day23.txt");
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = (0, empty_idx(&input[0]) as i32);
    let end = (
        input.len() as i32 - 1,
        empty_idx(input.last().unwrap()) as i32,
    );
    println!("{:?} {:?}", start, end);

    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let res = get_longest_path(&input, &mut visited, start, end);
    println!("{}", res.unwrap());
}
