// Advent of Code - Day 8: Haunted Wasteland Part 1

use std::collections::BTreeMap;

pub(crate) fn solve_part_one(input: &str) -> u32 {
    let mut input = input.lines();
    let instructions = input.next().unwrap().chars().cycle();
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
    instructions
        .enumerate()
        .try_fold("AAA", |current_node, (steps, instruction)| {
            match instruction {
                'L' => Some(network[current_node].0),
                'R' => Some(network[current_node].1),
                _ => None,
            }
            .map_or_else(
                || panic!("Invalid instruction encountered!"),
                |next_node| {
                    if next_node == "ZZZ" {
                        Err(steps as u32 + 1)
                    } else {
                        Ok(next_node)
                    }
                },
            )
        })
        .unwrap_err()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example1() {
        let example_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part1_example2() {
        let example_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 6);
    }
}
