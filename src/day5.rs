#![cfg(test)]

use std::ops::Range;

#[allow(unused)]
fn any_source_overlaps(mappings: &Vec<Vec<Vec<i64>>>) -> bool {
    for mapping in mappings {
        for i in 0..mapping.len() {
            let ri = mapping[i][1]..mapping[i][1] + mapping[i][2];
            for j in 0..mapping.len() {
                let rj = mapping[j][1]..mapping[j][1] + mapping[j][2];
                if i != j {
                    if ri.start < rj.end && ri.end > rj.start {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let lines: Vec<_> = input.lines().collect();

    let mut mappings = lines.split(|s| s.is_empty());
    let seeds: Vec<i64> = mappings.next().unwrap()[0]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mappings: Vec<Vec<Vec<i64>>> = mappings
        .map(|lines| {
            lines
                .iter()
                .skip(1)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    (seeds, mappings)
}

fn get_corresponding(mapping: &[Vec<i64>], value: i64) -> i64 {
    for row in mapping {
        let idx = value - row[1];
        if idx >= 0 && idx < row[2] {
            return row[0] + idx;
        }
    }
    value
}

#[test]
fn part1() {
    let input = include_str!("../input/day5.txt");
    let (seeds, mappings) = parse_input(input);

    let mut min = i64::MAX;
    for seed in seeds {
        let mut idx = seed;
        for mapping in &mappings {
            idx = get_corresponding(mapping, idx);
        }
        min = min.min(idx);
    }
    assert_eq!(min, 313045984);
}

#[derive(Clone, Copy, Debug)]
struct Mapping {
    from: i64,
    to: i64,
    diff: i64,
}

impl Mapping {
    fn same(from: i64, to: i64) -> Self {
        Mapping { from, to, diff: 0 }
    }
}

impl From<Vec<i64>> for Mapping {
    fn from(value: Vec<i64>) -> Self {
        Mapping {
            from: value[1],
            to: value[1] + value[2],
            diff: value[0] - value[1],
        }
    }
}

fn create_mappings(input: Vec<Vec<Vec<i64>>>) -> Vec<Vec<Mapping>> {
    let mut mappings: Vec<Vec<Mapping>> = input
        .into_iter()
        .map(|mapping| mapping.into_iter().map(Into::into).collect())
        .collect();
    for mapping in &mut mappings {
        mapping.sort_by_key(|v| v.from);

        let mut filled = Vec::new();
        filled.push(Mapping::same(i64::MIN, mapping[0].from));
        filled.push(mapping[0].clone());

        for pair in mapping.windows(2) {
            if pair[1].from > pair[0].to {
                filled.push(Mapping::same(pair[0].to, pair[1].from));
            }
            filled.push(pair[1]);
        }

        filled.push(Mapping::same(mapping.last().unwrap().to, i64::MAX));
        *mapping = filled;
    }
    mappings
}

fn clean_ranges(ranges: &mut Vec<Range<i64>>) {
    ranges.sort_by_key(|v| v.start);

    let mut res = Vec::with_capacity(ranges.len());
    for range in ranges.iter_mut() {
        match res.last_mut() {
            None => res.push(range.clone()),
            Some(last) => {
                if range.start <= last.end {
                    last.end = last.end.max(range.start);
                } else {
                    res.push(range.clone());
                }
            }
        }
    }
    *ranges = res;
}

fn map_ranges(mappings: &[Mapping], ranges: &[Range<i64>]) -> Vec<Range<i64>> {
    let mut res = Vec::new();

    let mut mapping_idx = 0;
    let mut range_idx = 0;
    while range_idx < ranges.len() {
        let range = &ranges[range_idx];
        let mapping = &mappings[mapping_idx];
        if range.start >= mapping.to {
            mapping_idx += 1;
        } else if range.end <= mapping.from {
            range_idx += 1;
        } else {
            let overlap = i64::max(range.start, mapping.from)..i64::min(range.end, mapping.to);
            res.push(overlap.start + mapping.diff..overlap.end + mapping.diff);

            if range.end < mapping.to {
                range_idx += 1;
            } else {
                mapping_idx += 1;
            }
        }
    }

    clean_ranges(&mut res);
    res
}

#[test]
fn part2() {
    let input = include_str!("../input/day5.txt");
    let (seeds, mappings) = parse_input(input);

    let mappings = create_mappings(mappings);
    let mut ranges: Vec<Range<i64>> = seeds.chunks_exact(2).map(|v| v[0]..v[0] + v[1]).collect();
    clean_ranges(&mut ranges);

    for mapping in &mappings {
        ranges = map_ranges(mapping, &ranges);
    }
    let min = ranges.iter().map(|v| v.start).min().unwrap();
    assert_eq!(min, 20283860)
}
