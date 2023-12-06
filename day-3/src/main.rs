// Advent of Code - Day 3: Gear Ratios

use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

#[derive(Default)]
struct EngineSchematic {
    parts: Vec<Part>,
    symbols: HashSet<(i32, i32)>,
    gears: HashSet<(i32, i32)>,
}

struct Part {
    number: u32,
    border: HashSet<(i32, i32)>,
}

impl Part {
    fn new(row: usize, col: usize, ch: char) -> Self {
        let row = row as i32;
        let col = col as i32;
        let border = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ]);
        Self {
            number: ch
                .to_digit(10)
                .expect("Char should be convertable to a digit."),
            border,
        }
    }

    fn add_digit(&mut self, row: usize, col: usize, ch: char) {
        let row = row as i32;
        let col = col as i32;
        self.border
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]); // add new right hand border
        self.number = self.number * 10
            + ch.to_digit(10)
                .expect("Char should be convertable to a digit."); // updates part number
    }
}

fn solve_part_one(input: &str) -> u32 {
    let mut engine_schematic = EngineSchematic::default();
    let mut current_part: Option<Part> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                if let Some(ref mut part) = current_part {
                    part.add_digit(row, col, ch)
                } else {
                    current_part = Some(Part::new(row, col, ch))
                }
            } else {
                if let Some(part) = current_part.take() {
                    engine_schematic.parts.push(part)
                }
                if ch != '.' {
                    engine_schematic.symbols.insert((row as i32, col as i32));
                }
            }
        }
    }
    engine_schematic
        .parts
        .iter()
        .filter_map(|part| {
            if part
                .border
                .intersection(&engine_schematic.symbols)
                .next()
                .is_some()
            {
                Some(part.number)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    let mut engine_schematic = EngineSchematic::default();
    let mut current_part: Option<Part> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                if let Some(ref mut part) = current_part {
                    part.add_digit(row, col, ch)
                } else {
                    current_part = Some(Part::new(row, col, ch))
                }
            } else {
                if let Some(part) = current_part.take() {
                    engine_schematic.parts.push(part)
                }
                if ch == '*' {
                    engine_schematic.gears.insert((row as i32, col as i32));
                }
            }
        }
    }
    engine_schematic
        .gears
        .iter()
        .flat_map(|gear| {
            let matches: Vec<u32> = engine_schematic
                .parts
                .iter()
                .filter_map(|part| {
                    if part.border.contains(gear) {
                        Some(part.number)
                    } else {
                        None
                    }
                })
                .collect();
            if matches.len() == 2 {
                Some(matches[0] * matches[1])
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer: u32 = solve_part_two(INPUT);
    println!("Part two: {part_two_answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 467835);
    }
}
