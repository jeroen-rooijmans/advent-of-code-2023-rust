// Advent of Code - Day 15: The Floor Will Be Lava Part 2

use crate::part1::count_energised_tiles;

pub(crate) fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let energised_tiles = (0..num_cols - 1)
        .map(|col_idx| count_energised_tiles(&grid, (0, col_idx), 'd'))
        .chain(
            (0..num_cols).map(|col_idx| count_energised_tiles(&grid, (num_rows - 1, col_idx), 'u')),
        )
        .chain((0..num_rows).map(|row_idx| count_energised_tiles(&grid, (row_idx, 0), 'r')))
        .chain(
            (0..num_rows).map(|row_idx| count_energised_tiles(&grid, (row_idx, num_cols - 1), 'l')),
        )
        .collect::<Vec<_>>();

    energised_tiles.iter().max().cloned().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 51);
    }
}
