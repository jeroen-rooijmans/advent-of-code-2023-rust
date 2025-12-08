// Advent of Code - Day 24: Never Tell Me The Odds

pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_answer = part1::solve(INPUT, 200_000_000_000_000.0..=400_000_000_000_000.0);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = part2::solve(INPUT);
    println!("Part two: {part_two_answer:?}");
}
