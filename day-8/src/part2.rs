// Advent of Code - Day 8: Haunted Wasteland Part 2

use std::collections::BTreeMap;

fn lcm(nums: &[usize]) -> usize {
    nums.iter().fold(1, |acc, &x| acc * x / gcd(acc, x))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) fn solve_part_two(input: &str) -> usize {
    let mut input = input.lines();
    let instructions = input.next().unwrap();
    let network: BTreeMap<&str, (&str, &str)> = input
        .skip(1)
        .filter_map(|line| {
            line.split_once(" = ").map(|(node, neighbours)| {
                (
                    node,
                    neighbours[1..neighbours.len() - 1]
                        .split_once(", ")
                        .unwrap(),
                )
            })
        })
        .collect();
    let starting_nodes = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<&str>>();
    let starting_nodes_steps = starting_nodes
        .iter()
        .map(|node| {
            instructions
                .chars()
                .cycle()
                .enumerate()
                .try_fold(*node, |current_node, (steps, instruction)| {
                    match instruction {
                        'L' => Some(network[current_node].0),
                        'R' => Some(network[current_node].1),
                        _ => None,
                    }
                    .map_or_else(
                        || panic!("Invalid instruction encountered!"),
                        |next_node| {
                            if next_node.ends_with('Z') {
                                Err(steps + 1)
                            } else {
                                Ok(next_node)
                            }
                        },
                    )
                })
                .unwrap_err()
        })
        .collect::<Vec<usize>>();
    lcm(&starting_nodes_steps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 6);
    }
}
