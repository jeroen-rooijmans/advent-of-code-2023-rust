// Advent of Code - Day 11: Cosmic Expansion Part 2

pub(crate) fn solve(input: &str, expansion: usize) -> usize {
    crate::part1::solve(input, expansion)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2_expansion_10() {
        let example_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let answer = crate::part2::solve(example_input, 10);
        assert_eq!(answer, 1030);
    }

    #[test]
    fn part2_expansion_100() {
        let example_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let answer = crate::part2::solve(example_input, 100);
        assert_eq!(answer, 8410);
    }
}
