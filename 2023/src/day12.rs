#![cfg(test)]

struct Input {
    row: Vec<char>,
    nums: Vec<usize>,
}

impl Input {
    fn try_place(&self, row_idx: usize, num: usize) -> Option<usize> {
        if row_idx + num > self.row.len()
            || self.row[row_idx..row_idx + num].iter().any(|v| *v == '.')
        {
            return None;
        }
        if row_idx + num == self.row.len() {
            Some(row_idx + num)
        } else {
            if self.row[row_idx + num] == '#' {
                None
            } else {
                Some(row_idx + num + 1)
            }
        }
    }

    fn num_options(&self, dp: &mut Vec<Vec<Option<u64>>>, row_idx: usize, num_idx: usize) -> u64 {
        if let Some(res) = dp[row_idx][num_idx] {
            return res;
        }

        let res = if num_idx >= self.nums.len() {
            if self.row[row_idx..].contains(&'#') {
                0
            } else {
                1
            }
        } else if row_idx + self.nums[num_idx] > self.row.len() {
            0
        } else if self.row[row_idx] == '#' {
            if let Some(next_idx) = self.try_place(row_idx, self.nums[num_idx]) {
                self.num_options(dp, next_idx, num_idx + 1)
            } else {
                0
            }
        } else if self.row[row_idx] == '?' {
            let mut res = self.num_options(dp, row_idx + 1, num_idx);
            if let Some(next_idx) = self.try_place(row_idx, self.nums[num_idx]) {
                res += self.num_options(dp, next_idx, num_idx + 1);
            }
            res
        } else {
            self.num_options(dp, row_idx + 1, num_idx)
        };
        dp[row_idx][num_idx] = Some(res);
        res
    }
}

#[test]
fn part1() {
    let input = include_str!("../input/day12.txt");

    let mut sum = 0;
    for line in input.lines() {
        let (row, check) = line.split_once(' ').unwrap();
        let row: Vec<char> = row.chars().collect();
        let nums: Vec<usize> = check.split(',').filter_map(|v| v.parse().ok()).collect();

        let mut dp = vec![vec![None; nums.len() + 1]; row.len() + 1];
        let input = Input { row, nums };
        sum += input.num_options(&mut dp, 0, 0);
    }
    assert_eq!(sum, 7163);
}

#[test]
fn part2() {
    let input = include_str!("../input/day12.txt");

    let mut sum = 0;
    for line in input.lines() {
        let (row, check) = line.split_once(' ').unwrap();
        let row = format!("{}?{}?{}?{}?{}", row, row, row, row, row);
        let check = format!("{},{},{},{},{}", check, check, check, check, check);
        let row: Vec<char> = row.chars().collect();
        let nums: Vec<usize> = check.split(',').filter_map(|v| v.parse().ok()).collect();

        let mut dp = vec![vec![None; nums.len() + 1]; row.len() + 1];
        let input = Input { row, nums };
        sum += input.num_options(&mut dp, 0, 0);
    }
    assert_eq!(sum, 17788038834112);
}
