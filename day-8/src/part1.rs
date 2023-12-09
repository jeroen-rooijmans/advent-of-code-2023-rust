// Advent of Code - Day 8: Haunted Wasteland Part 1

use std::collections::BTreeMap;

pub(crate) fn solve_part_one(input: &str) -> u32 {
    let mut input = input.lines();
    let instructions = input.next().unwrap().chars().cycle();
    let mut network: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    for line in input.skip(1) {
        if let Some((node, neighbours)) = line.split_once(" = ").map(|(node, neighbours)| {
            (
                node,
                neighbours[1..neighbours.len() - 1]
                    .split_once(", ")
                    .unwrap(),
            )
        }) {
            network.insert(node, neighbours);
        };
    }
    let mut current_node = "AAA";
    for (steps, instruction) in instructions.enumerate() {
        if current_node == "ZZZ" {
            return steps as u32;
        }
        match instruction {
            'L' => current_node = network[current_node].0,
            'R' => current_node = network[current_node].1,
            _ => panic!("Invalid instruction encountered!"),
        }
    }
    panic!("Should have found node ZZZ by now!");
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
