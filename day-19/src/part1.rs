// Advent of Code - Day 19: Aplenty Part 1

use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ratings: Vec<&str> = s[1..s.len() - 1].split(',').collect();

        Ok(Part {
            x: ratings[0].split_once('=').unwrap().1.parse().unwrap(),
            m: ratings[1].split_once('=').unwrap().1.parse().unwrap(),
            a: ratings[2].split_once('=').unwrap().1.parse().unwrap(),
            s: ratings[3].split_once('=').unwrap().1.parse().unwrap(),
        })
    }
}

impl Part {
    fn rating(&self, cat: char) -> usize {
        match cat {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn check_rule(rule: &str, part: &Part) -> bool {
    !rule.contains(':')
        || match (rule.chars().next().unwrap(), rule.chars().nth(1).unwrap()) {
            (cat, '<') if part.rating(cat) < rule[2..rule.find(':').unwrap()].parse().unwrap() => {
                true
            }
            (cat, '>') if part.rating(cat) > rule[2..rule.find(':').unwrap()].parse().unwrap() => {
                true
            }
            _ => false,
        }
}

fn accepted(workflows: &HashMap<&str, Vec<&str>>, part: &Part) -> bool {
    let mut current_workflow = "in";
    while current_workflow != "A" && current_workflow != "R" {
        current_workflow = workflows[current_workflow]
            .iter()
            .find(|&rule| check_rule(rule, part))
            .map(|&workflow| {
                if workflow.contains(':') {
                    workflow.split_once(':').unwrap().1
                } else {
                    workflow
                }
            })
            .unwrap();
    }
    current_workflow == "A"
}

pub(crate) fn solve(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').unwrap();
            let rules = rules
                .trim_end_matches('}')
                .split(',')
                .collect::<Vec<&str>>();
            (name, rules)
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    let parts = parts
        .lines()
        .map(|l| Part::from_str(l).expect("Should contain a valid Part!"))
        .collect::<Vec<Part>>();
    parts
        .iter()
        .filter(|&part| accepted(&workflows, part))
        .map(|part| part.total_rating())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 19114);
    }
}
