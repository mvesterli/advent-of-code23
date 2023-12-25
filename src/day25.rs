#![cfg(test)]

use rand::Rng;
use std::collections::{HashMap, VecDeque};

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut mapping = HashMap::new();
    let mut adj = Vec::new();
    for line in input.lines() {
        let (from, to) = line.split_once(':').unwrap();
        let from_idx = *mapping.entry(from).or_insert_with(|| {
            adj.push(Vec::new());
            adj.len() - 1
        });
        for to in to.trim().split_ascii_whitespace() {
            let to_idx = *mapping.entry(to).or_insert_with(|| {
                adj.push(Vec::new());
                adj.len() - 1
            });
            adj[from_idx].push(to_idx);
            adj[to_idx].push(from_idx);
        }
    }
    adj
}

fn find_path(
    graph: &Vec<Vec<usize>>,
    from_idx: usize,
    to_idx: usize,
    counts: &mut HashMap<(usize, usize), usize>,
) {
    let mut visited = vec![false; graph.len()];
    let mut path = vec![None; graph.len()];
    let mut q = VecDeque::new();
    q.push_back((from_idx, None));

    while let Some((front, from)) = q.pop_front() {
        if visited[front] {
            continue;
        }
        visited[front] = true;
        path[front] = from;
        if front == to_idx {
            break;
        }
        for to in &graph[front] {
            q.push_back((*to, Some(front)));
        }
    }
    let mut idx = to_idx;
    while let Some(from) = path[idx] {
        let pair = (idx.min(from), idx.max(from));
        *counts.entry(pair).or_default() += 1;
        idx = from
    }
}

fn get_size(graph: &Vec<Vec<usize>>, start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut q = VecDeque::new();
    q.push_back(start);

    let mut res = 0;
    while let Some(front) = q.pop_front() {
        if visited[front] {
            continue;
        }
        visited[front] = true;
        res += 1;
        for to in &graph[front] {
            q.push_back(*to);
        }
    }
    res
}

#[test]
fn part1() {
    let input = include_str!("../input/day25.txt");
    let mut graph = parse(input);
    let mut counts = HashMap::new();

    let mut rng = rand::thread_rng();
    for _ in 0..200 {
        let a = rng.gen_range(0..graph.len());
        let b = rng.gen_range(0..graph.len());
        find_path(&graph, a, b, &mut counts);
    }
    let mut top: Vec<_> = counts.into_iter().collect();
    top.sort_by_key(|v| v.1);
    for (edge, _) in &top[top.len() - 3..] {
        graph[edge.0].retain(|v| *v != edge.1);
        graph[edge.1].retain(|v| *v != edge.0);
    }
    let a = get_size(&graph, top.last().unwrap().0 .0);
    let b = get_size(&graph, top.last().unwrap().0 .1);
    println!("{}", a * b);
}
