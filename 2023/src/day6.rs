#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day6.txt");

    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<f32>().ok());
    let records = lines[1]
        .split_ascii_whitespace()
        .filter_map(|v| v.parse::<f32>().ok());

    let result: i32 = times
        .zip(records)
        .map(|(time, record)| {
            let record_hold_time = (time - (time * time - 4.0 * record).sqrt()) / 2.0;
            time as i32 - 2 * record_hold_time.floor() as i32 - 1
        })
        .product();
    assert_eq!(result, 4568778);
}

#[test]
fn part2() {
    let input = include_str!("../input/day6.txt");

    let lines = input
        .lines()
        .map(|v| v.replace(' ', ""))
        .collect::<Vec<_>>();
    let time: f64 = lines[0].split_once(':').unwrap().1.parse().unwrap();
    let record: f64 = lines[1].split_once(':').unwrap().1.parse().unwrap();

    let record_hold_time = (time - (time * time - 4.0 * record).sqrt()) / 2.0;
    let result = time as i32 - 2 * record_hold_time.floor() as i32 - 1;
    assert_eq!(result, 28973936);
}
