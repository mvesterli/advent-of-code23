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

#[test]
fn part2() {
    let input = include_str!("../input/day2.txt");

    let mut sum = 0;
    for game in input.lines() {
        let (_, game) = game.split_once(':').unwrap();

        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for grab in game.split(';') {
            for color in grab.split(',').map(str::trim) {
                let (num, color) = color.split_once(' ').unwrap();
                let num: i32 = num.parse().unwrap();

                match color {
                    "red" => max_r = max_r.max(num),
                    "green" => max_g = max_g.max(num),
                    "blue" => max_b = max_b.max(num),
                    _ => unreachable!(),
                };
            }
        }
        sum += max_r * max_g * max_b;
    }
    println!("{sum}");
}
