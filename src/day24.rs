#![cfg(test)]

use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Double3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, Debug)]
struct Hail {
    pos: Double3,
    dir: Double3,
}

impl From<&str> for Hail {
    fn from(line: &str) -> Self {
        let nums: Vec<f64> = line
            .split(|c: char| c != '-' && !c.is_ascii_digit())
            .filter_map(|v| v.parse().ok())
            .collect();
        Hail {
            pos: Double3 {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            },
            dir: Double3 {
                x: nums[3],
                y: nums[4],
                z: nums[5],
            },
        }
    }
}

impl Hail {
    fn position_at(&self, t: f64) -> Double3 {
        Double3 {
            x: self.pos.x + t * self.dir.x,
            y: self.pos.y + t * self.dir.y,
            z: self.pos.z + t * self.dir.z,
        }
    }

    fn line_intersection(&self, o: Hail) -> Option<(f64, f64)> {
        let det = self.dir.x * o.dir.y - self.dir.y * o.dir.x;
        if det == 0.0 {
            return None;
        }
        let t = ((self.pos.y - o.pos.y) * (o.dir.x) - (self.pos.x - o.pos.x) * o.dir.y) / det;
        let u = if o.dir.y != 0.0 {
            (self.pos.y + t * self.dir.y - o.pos.y) / o.dir.y
        } else {
            (self.pos.x + t * self.dir.x - o.pos.x) / o.dir.x
        };
        Some((t, u))
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day24.txt");
    let hails: Vec<Hail> = input.lines().map(|l| Hail::from(l)).collect();
    let min = 200000000000000.0;
    let max = 400000000000000.0;

    let mut res = 0;
    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            if let Some((t, u)) = hails[i].line_intersection(hails[j]) {
                if t >= 0.0 && u >= 0.0 {
                    let pos = hails[i].position_at(t);
                    if pos.x >= min && pos.y >= min && pos.x <= max && pos.y <= max {
                        res += 1;
                    }
                }
            }
        }
    }
    println!("{res}");
}

#[test]
fn part2() {
    let input = include_str!("../input/day24.txt");
    let hails: Vec<Hail> = input.lines().map(|l| Hail::from(l)).collect();

    let h0 = hails[0];
    let h1 = hails[1];
    let h2 = hails[2];

    // px py pz dx dy dz
    let co = array![
        [
            0.0,
            -h0.dir.z + h1.dir.z,
            h0.dir.y - h1.dir.y,
            0.0,
            h0.pos.z - h1.pos.z,
            -h0.pos.y + h1.pos.y,
        ],
        [
            h0.dir.z - h1.dir.z,
            0.0,
            -h0.dir.x + h1.dir.x,
            -h0.pos.z + h1.pos.z,
            0.0,
            h0.pos.x - h1.pos.x,
        ],
        [
            -h0.dir.y + h1.dir.y,
            h0.dir.x - h1.dir.x,
            0.0,
            h0.pos.y - h1.pos.y,
            -h0.pos.x + h1.pos.x,
            0.0,
        ],
        [
            0.0,
            -h0.dir.z + h2.dir.z,
            h0.dir.y - h2.dir.y,
            0.0,
            h0.pos.z - h2.pos.z,
            -h0.pos.y + h2.pos.y,
        ],
        [
            h0.dir.z - h2.dir.z,
            0.0,
            -h0.dir.x + h2.dir.x,
            -h0.pos.z + h2.pos.z,
            0.0,
            h0.pos.x - h2.pos.x,
        ],
        [
            -h0.dir.y + h2.dir.y,
            h0.dir.x - h2.dir.x,
            0.0,
            h0.pos.y - h2.pos.y,
            -h0.pos.x + h2.pos.x,
            0.0,
        ],
    ];
    let consts = array![
        h0.pos.z * h0.dir.y - h1.pos.z * h1.dir.y - h0.pos.y * h0.dir.z + h1.pos.y * h1.dir.z,
        h0.pos.x * h0.dir.z - h1.pos.x * h1.dir.z - h0.pos.z * h0.dir.x + h1.pos.z * h1.dir.x,
        h0.pos.y * h0.dir.x - h1.pos.y * h1.dir.x - h0.pos.x * h0.dir.y + h1.pos.x * h1.dir.y,
        h0.pos.z * h0.dir.y - h2.pos.z * h2.dir.y - h0.pos.y * h0.dir.z + h2.pos.y * h2.dir.z,
        h0.pos.x * h0.dir.z - h2.pos.x * h2.dir.z - h0.pos.z * h0.dir.x + h2.pos.z * h2.dir.x,
        h0.pos.y * h0.dir.x - h2.pos.y * h2.dir.x - h0.pos.x * h0.dir.y + h2.pos.x * h2.dir.y,
    ];
    let res = co.solve(&consts).unwrap();
    println!("{}", (res[0] + res[1] + res[2]).round() as i64);
}
