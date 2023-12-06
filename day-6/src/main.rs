// Advent of Code - Day 6: Wait For It

const INPUT: &str = include_str!("./input.txt");

fn calculate_distance(race_duration: &u64, charge_duration: u64) -> u64 {
    let move_duration = race_duration - charge_duration;
    move_duration * charge_duration
}

fn solve_part_one(input: &str) -> u64 {
    fn parse_line(line: &str) -> Vec<u64> {
        line.split_whitespace()
            .skip(1)
            .flat_map(|v| v.parse::<u64>())
            .collect::<Vec<u64>>()
    }
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let records = parse_line(lines.next().unwrap());
    times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            (1..time)
                .map(|charge| calculate_distance(&time, charge))
                .filter(|&distance| distance > record)
                .count() as u64
        })
        .product::<u64>()
}

fn solve_part_two(input: &str) -> u64 {
    fn parse_line(line: &str) -> u64 {
        line.chars()
            .filter(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let record = parse_line(lines.next().unwrap());
    (1..time)
        .map(|charge| calculate_distance(&time, charge))
        .filter(|&distance| distance > record)
        .count() as u64
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
        assert_eq!(answer, 288);
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 71503);
    }
}
