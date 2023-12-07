// Advent of Code - Day 7: Camel Cards

pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_answer = part1::solve_part_one(INPUT);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = part2::solve_part_two(INPUT);
    println!("Part two: {part_two_answer:?}");
}
