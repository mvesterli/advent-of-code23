#![cfg(test)]

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: Vec<Vec<usize>> = vec![Vec::new(); 100];
    for line in input.lines().take_while(|l| !l.is_empty()) {
        rules[line[0..2].parse::<usize>().unwrap()].push(line[3..].parse().unwrap());
    }
    let rows: Vec<Vec<usize>> = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.split(',').flat_map(|v| v.parse()).collect())
        .collect();
    (rules, rows)
}

fn is_valid(rules: &[Vec<usize>], pages: &[usize]) -> bool {
    let mut seen = [false; 100];
    for page in pages {
        for not_allowed_prev in &rules[*page] {
            if seen[*not_allowed_prev] {
                return false;
            }
        }
        seen[*page] = true;
    }
    true
}

fn fixup(rules: &[Vec<usize>], mut pages: Vec<usize>) -> Vec<usize> {
    let mut seen_positions = [None; 100];
    for (page_idx, page) in pages.iter().enumerate() {
        for not_allowed_prev in &rules[*page] {
            if let Some(seen) = seen_positions[*not_allowed_prev] {
                pages.swap(seen, page_idx);
                return fixup(rules, pages);
            }
        }
        seen_positions[*page] = Some(page_idx);
    }
    pages
}

#[test]
fn part1() {
    let input = include_str!("../input/day5.txt");
    let (rules, rows) = parse(input);
    let res: usize = rows
        .iter()
        .filter(|r| is_valid(&rules, r))
        .map(|r| r[r.len() / 2])
        .sum();
    assert_eq!(5275, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day5.txt");
    let (rules, rows) = parse(input);
    let result: usize = rows
        .into_iter()
        .filter(|r| !is_valid(&rules, r))
        .map(|r| fixup(&rules, r))
        .map(|r| r[r.len() / 2])
        .sum();
    assert_eq!(6191, result);
}
