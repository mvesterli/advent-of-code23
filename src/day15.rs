#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day15.txt");
    let res: i32 = input
        .trim()
        .split(',')
        .map(|s| s.bytes().fold(0, |acc, c| ((acc + c as i32) * 17) % 256))
        .sum();
    println!("{}", res);
}
