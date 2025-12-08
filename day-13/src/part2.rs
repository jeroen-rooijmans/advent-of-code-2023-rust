// Advent of Code - Day 13: Point of Incidence Part 2

fn find_horizontal_reflection(pattern: &[&[u8]]) -> Option<usize> {
    (0..pattern.len() - 1)
        .find(|&row_idx| {
            (0..=row_idx.min(pattern.len() - row_idx - 2))
                .map(|reflection_offset| {
                    let top_idx = row_idx - reflection_offset;
                    let bottom_idx = row_idx + 1 + reflection_offset;
                    (0..pattern[0].len())
                        .filter(|&col_idx| {
                            pattern[top_idx][col_idx] != pattern[bottom_idx][col_idx]
                        })
                        .count()
                })
                .sum::<usize>()
                == 1
        })
        .map(|n| n + 1)
}

fn find_vertical_reflection(pattern: &[&[u8]]) -> Option<usize> {
    (0..pattern[0].len() - 1)
        .find(|&col_idx| {
            (0..=col_idx.min(pattern[0].len() - col_idx - 2))
                .map(|reflection_offset| {
                    let left_idx = col_idx - reflection_offset;
                    let right_idx = col_idx + 1 + reflection_offset;
                    (0..pattern.len())
                        .filter(|&row_idx| {
                            pattern[row_idx][left_idx] != pattern[row_idx][right_idx]
                        })
                        .count()
                })
                .sum::<usize>()
                == 1
        })
        .map(|n| n + 1)
}

pub(crate) fn solve(input: &str) -> usize {
    let patterns = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>())
        .collect::<Vec<_>>();
    patterns
        .iter()
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
    fn part2() {
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
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 400);
    }
}
