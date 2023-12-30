// Advent of Code - Day 20: Pulse Propagation Part 2

use std::collections::{HashMap, VecDeque};

use crate::part1::Module;

pub(crate) fn solve(input: &str) -> usize {
    let mut module_configuration = HashMap::new();
    let mut state = HashMap::new();
    let mut second_last = None;
    for line in input.lines() {
        let (module, destination_modules) = line.split_once(" -> ").unwrap();
        let (module, name) = match module.chars().next().unwrap() {
            '%' => (Module::FlipFlop(false), &module[1..]),
            '&' => (Module::Conjunction(HashMap::new()), &module[1..]),
            'b' => (Module::Broadcaster, module),
            _ => unreachable!(),
        };
        if destination_modules.contains("rx") {
            second_last = Some(name);
        }
        module_configuration.insert(name, destination_modules.split(", ").collect::<Vec<&str>>());
        state.insert(name, module);
    }

    // Initialise memory for Conjunctions with low pulses
    for (module, connections) in &module_configuration {
        for con in connections {
            if let Some(Module::Conjunction(memory)) = state.get_mut(con) {
                memory.insert(module, false);
            }
        }
    }

    // Initialise counter for all modules that are connected to Conjunction that is connected to "rx"
    let second_last = second_last.unwrap();
    let mut memory_counter = {
        let Module::Conjunction(memory) = &state[second_last] else {
            unimplemented!()
        };
        memory
            .keys()
            .map(|&k| (k, None))
            .collect::<HashMap<&str, Option<usize>>>()
    };

    let mut q = VecDeque::new();
    for button_count in 1.. {
        q.push_back(("broadcaster", "button", false));
        while let Some((module, source, pulse)) = q.pop_front() {
            if pulse && module == second_last {
                let count = memory_counter.get_mut(source).unwrap();
                if count.is_none() {
                    *count = Some(button_count);
                    if memory_counter.values().all(|c| c.is_some()) {
                        return memory_counter.values().map(|c| c.unwrap()).product();
                    }
                }
            }
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
    unreachable!()
}
