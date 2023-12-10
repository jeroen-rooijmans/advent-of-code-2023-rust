// Advent of Code - Day 9: Mirage Maintenance Part 1

pub(crate) fn differences(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

pub(crate) fn solve_part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let oasis_report = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Should be parsable."))
                .collect::<Vec<i64>>();
            let mut diffs = vec![oasis_report];
            while !diffs.last().unwrap().iter().all(|&v| v == 0) {
                diffs.push(differences(diffs.last().unwrap()));
            }
            diffs
                .iter()
                .rev()
                .fold(0, |acc, diff| diff.last().unwrap() + acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 114);
    }
}
