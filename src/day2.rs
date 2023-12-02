#![cfg(test)]

fn is_grab_ok(game: &str) -> bool {
    for grab in game.split(';') {
        for color in grab.split(',').map(str::trim) {
            let (num, color) = color.split_once(' ').unwrap();
            let num: i32 = num.parse().unwrap();

            let ok = match color {
                "red" => num <= 12,
                "green" => num <= 13,
                "blue" => num <= 14,
                _ => unreachable!(),
            };
            if !ok {
                return false;
            }
        }
    }
    true
}

#[test]
fn part1() {
    let input = include_str!("../input/day2.txt");

    let mut sum = 0;
    for game in input.lines() {
        let (name, game) = game.split_once(':').unwrap();
        let id: i32 = name.split_once(' ').unwrap().1.parse().unwrap();
        if is_grab_ok(game) {
            sum += id;
        }
    }
    println!("{sum}");
}
