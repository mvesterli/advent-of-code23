#![cfg(test)]

use std::collections::HashMap;

fn empty_idx(line: &[char]) -> usize {
    line.iter().enumerate().find(|v| *v.1 == '.').unwrap().0
}

fn allowed_directions(cell: char) -> &'static [(i32, i32)] {
    match cell {
        '.' => &[(-1, 0), (0, -1), (0, 1), (1, 0)],
        '^' => &[(-1, 0)],
        'v' => &[(1, 0)],
        '<' => &[(0, -1)],
        '>' => &[(0, 1)],
        _ => unreachable!(),
    }
}

fn is_inside(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < grid.len() as i32 && pos.1 < grid[0].len() as i32
}

fn is_valid(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    is_inside(grid, pos) && grid[pos.0 as usize][pos.1 as usize] != '#'
}

fn is_node(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    if grid[pos.0 as usize][pos.1 as usize] == '#' {
        return false;
    }
    let count = allowed_directions('.')
        .iter()
        .filter(|dir| grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] != '#')
        .count();
    count > 2
}

fn get_neighbor(
    grid: &Vec<Vec<char>>,
    nodes: &HashMap<(i32, i32), usize>,
    allow_slopes: bool,
    node: (i32, i32),
    prev: (i32, i32),
    dist: usize,
) -> Option<((i32, i32), usize)> {
    if !is_valid(grid, node) {
        return None;
    }
    if nodes.contains_key(&node) {
        return Some((node, dist));
    }
    let current = if allow_slopes {
        '.'
    } else {
        grid[node.0 as usize][node.1 as usize]
    };
    allowed_directions(current)
        .iter()
        .map(|dir| (node.0 + dir.0, node.1 + dir.1))
        .filter(|next| *next != prev)
        .find_map(|next| get_neighbor(grid, nodes, allow_slopes, next, node, dist + 1))
}

#[derive(Debug)]
struct Edge {
    to: usize,
    dist: usize,
}

fn build_graph(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    allow_slopes: bool,
) -> Vec<Vec<Edge>> {
    let mut mapping = HashMap::new();
    let mut graph = Vec::new();
    mapping.insert(start, 0);
    mapping.insert(end, 1);
    graph.push(Vec::new());
    graph.push(Vec::new());
    for row in 1..grid.len() as i32 - 1 {
        for col in 1..grid[0].len() as i32 - 1 {
            if is_node(grid, (row, col)) {
                mapping.insert((row, col), graph.len());
                graph.push(Vec::new());
            }
        }
    }
    for node in mapping.keys() {
        for dir in allowed_directions('.') {
            let next = (node.0 + dir.0, node.1 + dir.1);
            if let Some(neighbor) = get_neighbor(grid, &mapping, allow_slopes, next, *node, 1) {
                graph[mapping[node]].push(Edge {
                    to: mapping[&neighbor.0],
                    dist: neighbor.1,
                });
            }
        }
    }
    graph
}

fn get_longest_path(
    graph: &Vec<Vec<Edge>>,
    visited: &mut Vec<bool>,
    node: usize,
    end: usize,
) -> Option<usize> {
    if visited[node] {
        return None;
    }
    if node == end {
        return Some(0);
    }

    let mut res = None;
    visited[node] = true;
    for edge in &graph[node] {
        let option = get_longest_path(graph, visited, edge.to, end);
        if let Some(option) = option {
            if let Some(ref mut res) = res {
                if option + edge.dist > *res {
                    *res = option + edge.dist
                }
            } else {
                res = Some(option + edge.dist);
            }
        }
    }
    visited[node] = false;
    res
}

fn solve(input: &str, allow_slopes: bool) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = (0, empty_idx(&input[0]) as i32);
    let end = (
        input.len() as i32 - 1,
        empty_idx(input.last().unwrap()) as i32,
    );
    let graph = build_graph(&input, start, end, allow_slopes);
    let mut visited = vec![false; graph.len()];
    get_longest_path(&graph, &mut visited, 0, 1).unwrap()
}

#[test]
fn part1() {
    let input = include_str!("../input/day23.txt");
    assert_eq!(solve(input, false), 2294);
}

#[test]
fn part2() {
    let input = include_str!("../input/day23.txt");
    assert_eq!(solve(input, true), 6418);
}
