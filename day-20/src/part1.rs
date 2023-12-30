// Advent of Code - Day 20: Pulse Propagation Part 1

use std::collections::{HashMap, VecDeque};

pub(crate) enum Module<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}

pub(crate) fn solve(input: &str) -> usize {
    let mut module_configuration = HashMap::new();
    let mut state = HashMap::new();
    for line in input.lines() {
        let (module, destination_modules) = line.split_once(" -> ").unwrap();
        let (module, name) = match module.chars().next().unwrap() {
            '%' => (Module::FlipFlop(false), &module[1..]),
            '&' => (Module::Conjunction(HashMap::new()), &module[1..]),
            'b' => (Module::Broadcaster, module),
            _ => unreachable!(),
        };
        module_configuration.insert(name, destination_modules.split(", ").collect::<Vec<&str>>());
        state.insert(name, module);
    }

    // Initialise Conjunctions
    for (module, connections) in &module_configuration {
        for con in connections {
            if let Some(Module::Conjunction(memory)) = state.get_mut(con) {
                memory.insert(module, false);
            }
        }
    }

    let mut q = VecDeque::new();
    let mut pulses = [0, 0];
    for _ in 0..1000 {
        q.push_back(("broadcaster", "button", false));
        while let Some((module, source, pulse)) = q.pop_front() {
            pulses[pulse as usize] += 1;
            let next_pulse = match state.get_mut(module) {
                Some(Module::FlipFlop(_)) if pulse => None,
                Some(Module::FlipFlop(status)) => {
                    *status = !*status;
                    Some(*status)
                }
                Some(Module::Conjunction(memory)) => {
                    memory.insert(source, pulse);
                    Some(memory.values().any(|&b| !b))
                }
                Some(Module::Broadcaster) => Some(false),
                None => None,
            };
            if let Some(next_modules) = &module_configuration.get(module) {
                if let Some(pulse) = next_pulse {
                    q.extend(next_modules.iter().map(|&next| (next, module, pulse)))
                }
            }
        }
    }
    pulses[0] * pulses[1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 32000000);
    }

    #[test]
    fn part1_interesting() {
        let example_input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 11687500);
    }
}
