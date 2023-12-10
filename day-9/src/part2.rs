// Advent of Code - Day 9: Mirage Maintenance Part 2

pub(crate) fn solve_part_two(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let oasis_report = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Should be parsable."))
                .collect::<Vec<i64>>();
            let mut diffs = vec![oasis_report];
            while !diffs.last().unwrap().iter().all(|&v| v == 0) {
                diffs.push(crate::part1::differences(diffs.last().unwrap()));
            }
            diffs.iter().rev().fold(0, |acc, diff| diff[0] - acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 2);
    }
}
