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
    println!("{}", result);
}
