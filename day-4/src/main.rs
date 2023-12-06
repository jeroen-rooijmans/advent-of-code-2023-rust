// Advent of Code - Day 4: Scratchcards

use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

struct Scratchcard {
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Scratchcard {
    fn count_winning_numbers(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .count() as u32
    }

    fn value(&self) -> u32 {
        2_u32.pow(self.count_winning_numbers()) >> 1
    }
}

fn solve_part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning_numbers_seq, card_numbers_seq) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning_numbers_seq
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let card_numbers: HashSet<u32> = card_numbers_seq
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            Scratchcard {
                winning_numbers,
                card_numbers,
            }
        })
        .map(|scratchcard| scratchcard.value())
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    let original_cards: Vec<Scratchcard> = input
        .lines()
        .map(|line| {
            let (winning_numbers_seq, card_numbers_seq) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_numbers: HashSet<u32> = winning_numbers_seq
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let card_numbers: HashSet<u32> = card_numbers_seq
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            Scratchcard {
                winning_numbers,
                card_numbers,
            }
        })
        .collect();
    let mut card_counter: Vec<u32> = vec![1u32; original_cards.len()];
    original_cards.iter().enumerate().for_each(|(idx, card)| {
        (idx + 1..idx + 1 + card.count_winning_numbers() as usize).for_each(|i| {
            card_counter[i] += card_counter[idx];
        });
    });
    card_counter.iter().sum()
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
        assert_eq!(answer, 13);
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 30);
    }
}
