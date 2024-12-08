#![cfg(test)]

use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
enum NodeType<'a> {
    Flip(bool),
    Conj(HashMap<&'a str, bool>),
}

#[derive(Clone, Debug)]
struct Node<'a> {
    name: &'a str,
    dest: Vec<&'a str>,
    kind: NodeType<'a>,
}

#[derive(Clone, Copy, Debug)]
struct Pulse<'a> {
    from: &'a str,
    is_high: bool,
    to: &'a str,
}

impl<'a> Node<'a> {
    fn receive(&mut self, pulse: &Pulse<'a>, q: &mut VecDeque<Pulse<'a>>) {
        match &mut self.kind {
            NodeType::Flip(ref mut v) => {
                if !pulse.is_high {
                    let send_high = !*v;
                    *v = !*v;
                    for node in &self.dest {
                        q.push_back(Pulse {
                            from: self.name,
                            is_high: send_high,
                            to: node,
                        });
                    }
                }
            }
            NodeType::Conj(ref mut v) => {
                v.insert(pulse.from, pulse.is_high);
                let send_high = !v.values().all(|v| *v);
                for node in &self.dest {
                    q.push_back(Pulse {
                        from: self.name,
                        is_high: send_high,
                        to: node,
                    })
                }
            }
        }
    }

    fn add_connection(&mut self, name: &'a str) {
        match &mut self.kind {
            NodeType::Conj(ref mut map) => {
                map.insert(name, false);
            }
            _ => {}
        }
    }
}

fn parse(input: &str) -> (Vec<&str>, HashMap<&str, Node>) {
    let mut broadcast = None;
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let (name, dest) = line.split_once(" -> ").unwrap();
        let dest = dest.split(", ").collect();

        if name.starts_with("%") {
            nodes.insert(
                &name[1..],
                Node {
                    name: &name[1..],
                    kind: NodeType::Flip(false),
                    dest,
                },
            );
        } else if name.starts_with("&") {
            nodes.insert(
                &name[1..],
                Node {
                    name: &name[1..],
                    kind: NodeType::Conj(HashMap::new()),
                    dest,
                },
            );
        } else {
            broadcast = Some(dest);
        }
    }
    let nodes_vec: Vec<Node> = nodes.values().cloned().collect();
    for from in nodes_vec {
        for to in from.dest {
            if let Some(to) = nodes.get_mut(to) {
                to.add_connection(from.name);
            }
        }
    }
    (broadcast.unwrap(), nodes)
}

#[test]
fn part1() {
    let input = include_str!("../input/day20.txt");
    let (broadcast, mut nodes) = parse(input);

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        low += 1; // button to broadcast.
        let mut q = VecDeque::new();
        for node in &broadcast {
            q.push_back(Pulse {
                from: "broadcast",
                to: node,
                is_high: false,
            });
        }
        while let Some(pulse) = q.pop_front() {
            if pulse.is_high {
                high += 1;
            } else {
                low += 1
            }
            if let Some(to) = nodes.get_mut(pulse.to) {
                to.receive(&pulse, &mut q);
            }
        }
    }
    assert_eq!(low * high, 788848550);
}

fn solve2(input: &str) -> i64 {
    let (broadcast, mut nodes) = parse(input);

    let mut iters = HashMap::new();
    for presses in 1_i64.. {
        if iters.len() == 4 {
            return iters.into_values().reduce(num::integer::lcm).unwrap();
        }

        let mut q = VecDeque::new();
        for node in &broadcast {
            q.push_back(Pulse {
                from: "broadcast",
                to: node,
                is_high: false,
            });
        }
        while let Some(pulse) = q.pop_front() {
            if let Some(to) = nodes.get_mut(pulse.to) {
                to.receive(&pulse, &mut q);
            }
            if let NodeType::Conj(m) = &nodes["vd"].kind {
                for value in m {
                    if *value.1 && !iters.contains_key(*value.0) {
                        iters.insert(*value.0, presses);
                    }
                }
            }
        }
    }
    0
}
#[test]
fn part2() {
    let input = include_str!("../input/day20.txt");
    assert_eq!(solve2(input), 228300182686739);
}
