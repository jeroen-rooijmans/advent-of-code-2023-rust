// Advent of Code - Day 7:

const INPUT: &str = include_str!("./input.txt");

fn solve_part_one(input: &str) -> u32 {
    todo!()
}

fn solve_part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two: {part_two_answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, todo!());
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, todo!());
    }
}
