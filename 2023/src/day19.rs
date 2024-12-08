#![cfg(test)]

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Rule<'a> {
    var: char,
    op: char,
    val: i32,
    workflow: &'a str,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)([<>])(\d+):(\w+)").unwrap());
        let [var, op, val, workflow] = RE.captures(value).unwrap().extract::<4>().1;
        Rule {
            var: var.chars().next().unwrap(),
            op: op.chars().next().unwrap(),
            val: val.parse().unwrap(),
            workflow,
        }
    }
}

impl<'a> Rule<'a> {
    fn matches(&self, part: &HashMap<char, i32>) -> bool {
        match self.op {
            '>' => part[&self.var] > self.val,
            '<' => part[&self.var] < self.val,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback: &'a str,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(.*),(\w+)\}").unwrap());
        let [name, rules, fallback] = RE.captures(value).unwrap().extract::<3>().1;
        Workflow {
            name,
            rules: rules.split(',').map(|s| Rule::from(s)).collect(),
            fallback,
        }
    }
}

impl<'a> Workflow<'a> {
    pub fn apply(&self, part: &HashMap<char, i32>) -> &'a str {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.workflow;
            }
        }
        self.fallback
    }
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<HashMap<char, i32>>) {
    let input: Vec<_> = input.lines().collect();

    let mut parts = input.split(|l| l.is_empty());
    let workflows: Vec<Workflow> = parts
        .next()
        .unwrap()
        .iter()
        .map(|line| (*line).into())
        .collect();
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let values: Vec<HashMap<char, i32>> = parts
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap().extract::<4>().1;
            "xmas"
                .chars()
                .zip(captures.into_iter().filter_map(|s| s.parse().ok()))
                .collect()
        })
        .collect();
    (workflows, values)
}

#[test]
fn part1() {
    let input = include_str!("../input/day19.txt");

    let (workflows, parts) = parse(input);
    let workflows: HashMap<&str, Workflow> = workflows.into_iter().map(|v| (v.name, v)).collect();

    let mut result = 0;
    for part in parts {
        let mut workflow_name = "in";
        loop {
            if workflow_name == "A" {
                result += part.values().sum::<i32>();
                break;
            }
            if workflow_name == "R" {
                break;
            }
            workflow_name = workflows[workflow_name].apply(&part);
        }
    }
    assert_eq!(result, 319295);
}

fn solve(
    workflows: &HashMap<&str, Workflow>,
    name: &str,
    mut options: HashMap<char, (u64, u64)>,
) -> u64 {
    if options.values().any(|v| v.0 > v.1) {
        return 0;
    }
    if name == "A" {
        return options.values().map(|v| v.1 - v.0 + 1).product();
    }
    if name == "R" {
        return 0;
    }

    let mut res = 0;
    for rule in &workflows[name].rules {
        let current = options[&rule.var];
        match rule.op {
            '>' => {
                options.insert(rule.var, (rule.val as u64 + 1, current.1));
                res += solve(workflows, rule.workflow, options.clone());
                options.insert(rule.var, (current.0, rule.val as u64));
            }
            '<' => {
                options.insert(rule.var, (current.0, rule.val as u64 - 1));
                res += solve(workflows, rule.workflow, options.clone());
                options.insert(rule.var, (rule.val as u64, current.1));
            }
            _ => {
                unreachable!()
            }
        }
    }
    res += solve(workflows, workflows[name].fallback, options);
    res
}

#[test]
fn part2() {
    let input = include_str!("../input/day19.txt");
    let (workflows, _) = parse(input);
    let workflows = workflows.into_iter().map(|v| (v.name, v)).collect();
    let options = "xmas".chars().map(|c| (c, (1, 4000))).collect();
    let result = solve(&workflows, "in", options);
    assert_eq!(result, 110807725108076);
}
