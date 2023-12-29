// Advent of Code - Day 19: Aplenty Part 2

use std::collections::HashMap;

fn apply_workflows(
    workflows: &HashMap<&str, (Vec<&str>, &str)>,
    mut part: [Vec<usize>; 4],
    workflow_id: &str,
) -> usize {
    if workflow_id == "A" {
        return part.iter().map(|v| v.len()).product();
    } else if workflow_id == "R" {
        return 0;
    }
    let mut acc = 0;
    let workflow = &workflows[workflow_id];
    for rule in &workflow.0 {
        if rule.contains(':') {
            let mut rule_chars = rule.chars().take(2);
            let cat = rule_chars.next().unwrap();
            let comp = rule_chars.next().unwrap();
            let val = rule[2..rule.find(':').unwrap()].parse().unwrap();
            let next_workflow = &rule[rule.find(':').unwrap() + 1..];
            let i = "xmas".chars().position(|c| c == cat).unwrap();
            let mut new_part = part.clone();
            (new_part[i], part[i]) =
                part[i]
                    .iter()
                    .partition(|&&n| if comp == '<' { n < val } else { n > val });
            acc += apply_workflows(workflows, new_part, next_workflow);
        } else {
            acc += apply_workflows(workflows, part.clone(), rule);
        }
    }
    acc += apply_workflows(workflows, part, workflow.1);
    acc
}

pub(crate) fn solve(input: &str) -> usize {
    let workflows = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').unwrap();
            let mut rules = rules
                .trim_end_matches('}')
                .split(',')
                .collect::<Vec<&str>>();
            let final_workflow = rules.pop().unwrap();
            (name, (rules, final_workflow))
        })
        .collect::<HashMap<&str, (Vec<&str>, &str)>>();
    let part: [Vec<usize>; 4] = std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>());
    apply_workflows(&workflows, part, "in")
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
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
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 167409079868000);
    }
}
