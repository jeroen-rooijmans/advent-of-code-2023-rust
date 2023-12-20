// Advent of Code - Day 16: The Floor Will Be Lava

pub mod part1;
pub mod part2;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_answer = part1::solve(INPUT);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = part2::solve(INPUT);
    println!("Part two: {part_two_answer:?}");
}
