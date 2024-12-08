#![cfg(test)]

use std::collections::BinaryHeap;

fn graph_idx(input: &Vec<Vec<u32>>, row: i32, col: i32) -> Option<usize> {
    if row < 0 || col < 0 || row >= input.len() as i32 || col >= input[0].len() as i32 {
        None
    } else {
        Some((row as usize * input[0].len() + col as usize) * 2)
    }
}

#[derive(Debug)]
struct Edge {
    cost: u32,
    node: usize,
}

fn extend_adj_list(
    input: &Vec<Vec<u32>>,
    list: &mut Vec<Edge>,
    min: i32,
    max: i32,
    is_vertical: usize,
    f: impl Fn(i32) -> (i32, i32),
) {
    let mut cost = 0;
    for i in 1..=max {
        let (row, col) = f(i);
        if let Some(node) = graph_idx(&input, row, col) {
            cost += input[row as usize][col as usize];
            if i >= min {
                list.push(Edge {
                    node: node + is_vertical,
                    cost,
                })
            }
        }
    }
}

fn build_graph(input: &Vec<Vec<u32>>, min: i32, max: i32) -> Vec<Vec<Edge>> {
    let mut adj_list = Vec::new();
    for row in 0..input.len() as i32 {
        for col in 0..input[0].len() as i32 {
            let mut vertical = Vec::new();
            extend_adj_list(input, &mut vertical, min, max, 1, |i| (row - i, col));
            extend_adj_list(input, &mut vertical, min, max, 1, |i| (row + i, col));
            adj_list.push(vertical);

            let mut horizontal = Vec::new();
            extend_adj_list(input, &mut horizontal, min, max, 0, |i| (row, col - i));
            extend_adj_list(input, &mut horizontal, min, max, 0, |i| (row, col + i));
            adj_list.push(horizontal);
        }
    }
    adj_list
}

fn parse_graph(input: &str, min: i32, max: i32) -> Vec<Vec<Edge>> {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|v| v.to_digit(10)).collect())
        .collect();
    build_graph(&input, min, max)
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_dist(adj_list: &[Vec<Edge>]) -> Option<u32> {
    let mut dists = vec![u32::MAX; adj_list.len()];

    let mut pq = BinaryHeap::new();
    dists[0] = 0;
    dists[1] = 0;
    pq.push(State { node: 0, cost: 0 });
    pq.push(State { node: 1, cost: 0 });

    while let Some(state) = pq.pop() {
        if state.node >= adj_list.len() - 2 {
            return Some(state.cost);
        }

        if state.cost > dists[state.node] {
            continue;
        }

        for edge in &adj_list[state.node] {
            let next = State {
                cost: state.cost + edge.cost,
                node: edge.node,
            };
            if next.cost < dists[next.node] {
                pq.push(next);
                dists[next.node] = next.cost;
            }
        }
    }
    None
}

#[test]
fn part1() {
    let input = include_str!("../input/day17.txt");
    assert_eq!(get_dist(&parse_graph(input, 1, 3)).unwrap(), 936);
}

#[test]
fn part2() {
    let input = include_str!("../input/day17.txt");
    assert_eq!(get_dist(&parse_graph(input, 4, 10)).unwrap(), 1157);
}
