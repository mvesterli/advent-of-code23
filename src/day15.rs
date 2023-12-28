#![cfg(test)]

use regex::Regex;

#[test]
fn part1() {
    let input = include_str!("../input/day15.txt");
    let res: i32 = input
        .trim()
        .split(',')
        .map(|s| s.bytes().fold(0, |acc, c| ((acc + c as i32) * 17) % 256))
        .sum();
    assert_eq!(res, 509152);
}

#[test]
fn part2() {
    let input = include_str!("../input/day15.txt");
    let re = Regex::new(r"(\w+)(-|=)(\d*)").unwrap();

    let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
    for instruction in input.trim().split(',') {
        let parts = re.captures(instruction).unwrap().extract::<3>().1;
        let box_idx = parts[0]
            .bytes()
            .fold(0, |acc, c| (acc + c as usize) * 17 % 256);
        if parts[1] == "=" {
            if let Some(idx) = boxes[box_idx].iter_mut().find(|p| p.0 == parts[0]) {
                idx.1 = parts[2].parse().unwrap();
            } else {
                boxes[box_idx].push((parts[0], parts[2].parse().unwrap()));
            }
        } else {
            boxes[box_idx].retain(|p| p.0 != parts[0]);
        }
    }
    let res = boxes
        .iter()
        .enumerate()
        .map(|(idx, b)| {
            (idx + 1)
                * b.iter()
                    .enumerate()
                    .map(|(idx, v)| (idx + 1) * v.1 as usize)
                    .sum::<usize>()
        })
        .sum::<usize>();
    assert_eq!(res, 244403);
}
