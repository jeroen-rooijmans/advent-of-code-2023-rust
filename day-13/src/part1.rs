// Advent of Code - Day 13: Point of Incidence Part 1

pub(crate) fn find_horizontal_reflection(pattern: &str) -> Option<usize> {
    let rows = pattern.lines().collect::<Vec<&str>>();
    rows.windows(2)
        .enumerate()
        .filter(|(_, ls)| ls[0] == ls[1])
        .find_map(|(idx, _ls)| {
            let top_reflection = rows[0..=idx].iter().rev();
            let bottom_reflection = rows[idx + 1..].iter();
            top_reflection
                .zip(bottom_reflection)
                .all(|(a, b)| a == b)
                .then_some(idx + 1)
        })
}

pub(crate) fn find_vertical_reflection(pattern: &str) -> Option<usize> {
    let rows = pattern
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let cols = (0..rows[0].len())
        .map(|col_idx| rows.iter().map(|row| row[col_idx]).collect::<Vec<char>>())
        .collect::<Vec<_>>();
    cols.windows(2)
        .enumerate()
        .filter(|(_, cs)| cs[0] == cs[1])
        .find_map(|(idx, _cs)| {
            let left_reflection = cols[0..=idx].iter().rev();
            let right_reflection = cols[idx + 1..].iter();
            left_reflection
                .zip(right_reflection)
                .all(|(a, b)| a == b)
                .then_some(idx + 1)
        })
}

pub(crate) fn solve(input: &str) -> usize {
    let patterns = input.split("\n\n");
    patterns
        .map(|pattern| {
            find_horizontal_reflection(pattern)
                .map(|r| r * 100)
                .or_else(|| find_vertical_reflection(pattern))
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 405);
    }
}
