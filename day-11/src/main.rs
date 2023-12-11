// Advent of Code - Day 11: Cosmic Expansion

pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_answer = part1::solve(INPUT, 2);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = part2::solve(INPUT, 1_000_000);
    println!("Part two: {part_two_answer:?}");
}
