#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day9.txt");
    let mut disk = Vec::new();

    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap();
        for _ in 0..num {
            if i % 2 == 0 {
                disk.push(Some(i / 2));
            } else {
                disk.push(None);
            }
        }
    }

    let mut next_free = 0;
    for i in (0..disk.len()).rev() {
        if disk[i].is_some() {
            while next_free < i && disk[next_free].is_some() {
                next_free += 1;
            }
            if i > next_free {
                disk.swap(i, next_free);
                next_free += 1;
            }
        }
    }

    let result: usize = disk
        .iter()
        .enumerate()
        .filter_map(|v| v.1.map(|num| num * v.0))
        .sum();
    assert_eq!(6332189866718, result);
}

struct Region {
    position: usize,
    size: usize,
}

struct File {
    id: usize,
    region: Region,
}

#[test]
fn part2() {
    let input = include_str!("../input/day9.txt");

    let mut files = Vec::new();
    let mut free_spaces = Vec::new();

    let mut pos = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(File {
                id: i / 2,
                region: Region {
                    position: pos,
                    size: num,
                },
            });
        } else {
            free_spaces.push(Region {
                position: pos,
                size: num,
            });
        }
        pos += num;
    }
    for (index, file) in files.iter_mut().enumerate().rev() {
        for free_space in &mut free_spaces[..index] {
            if free_space.size >= file.region.size {
                file.region.position = free_space.position;
                free_space.position += file.region.size;
                free_space.size -= file.region.size;
                break;
            }
        }
    }
    let mut res = 0;
    for file in files {
        for i in 0..file.region.size {
            res += file.id * (file.region.position + i);
        }
    }
    assert_eq!(6353648390778, res);
}
