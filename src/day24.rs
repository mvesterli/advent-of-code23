#![cfg(test)]

#[derive(Copy, Clone, Debug)]
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

    fn lines_intersects_within(&self, o: Hail, min: f64, max: f64) -> bool {
        let det = self.dir.x * o.dir.y - self.dir.y * o.dir.x;
        if det == 0.0 {
            return false;
        }
        let t = ((self.pos.y - o.pos.y) * (o.dir.x) - (self.pos.x - o.pos.x) * o.dir.y) / det;
        let u = if o.dir.y != 0.0 {
            (self.pos.y + t * self.dir.y - o.pos.y) / o.dir.y
        } else {
            (self.pos.x + t * self.dir.x - o.pos.x) / o.dir.x
        };
        if t < 0.0 || u < 0.0 {
            return false;
        }
        let pos = self.position_at(t);
        pos.x >= min && pos.y >= min && pos.x <= max && pos.y <= max
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day24.txt");
    let hails: Vec<Hail> = input.lines().map(|l| Hail::from(l)).collect();

    let mut res = 0;
    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            if hails[i].lines_intersects_within(hails[j], 200000000000000.0, 400000000000000.0) {
                res += 1;
            }
        }
    }
    println!("{res}");
}
