#![cfg(test)]
use regex::Regex;

#[test]
fn part1() {
    let input = include_str!("../input/day3.txt");
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
    let result: i32 = re
        .captures_iter(input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum();
    assert_eq!(168539636, result);
}

#[test]
fn part2() {
    let input = include_str!("../input/day3.txt");
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)"#).unwrap();

    let mut enabled = true;
    let mut result = 0;
    for m in re.captures_iter(input) {
        match m.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    result += m.get(1).unwrap().as_str().parse::<i32>().unwrap()
                        * m.get(2).unwrap().as_str().parse::<i32>().unwrap()
                }
            }
        }
    }
    assert_eq!(97529391, result);
}
