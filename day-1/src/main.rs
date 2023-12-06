// Advent of Code - Day 1: Trebucket?!

use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn solve_part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .find_map(|char| char.to_digit(10))
                .expect("Each line should contain a valid digit.")
                * 10
                + line
                    .chars()
                    .rev()
                    .find_map(|char| char.to_digit(10))
                    .expect("Each line should contain a valid digit.")
        })
        .sum::<u32>()
}

fn solve_part_two(input: &str) -> u32 {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| {
            digits[(0..line.len())
                .find_map(|i| digits.keys().find(|key| line[i..].starts_with(*key)))
                .expect("Each line should contain a valid digit.")]
                * 10
                + digits[(0..line.len())
                    .find_map(|i| {
                        digits
                            .keys()
                            .find(|key| line[..(line.len() - i)].ends_with(*key))
                    })
                    .expect("Each line should contain a valid digit.")]
        })
        .sum()
}

fn main() {
    let part_one_answer: u32 = solve_part_one(INPUT);
    println!("Part one:\nSum of all calibration values: {part_one_answer:?}");
    let part_two_answer: u32 = solve_part_two(INPUT);
    println!("Part two:\nSum of all calibration values: {part_two_answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 142);
    }

    #[test]
    fn part2() {
        let example_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 281);
    }
}
