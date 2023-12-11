// Advent of Code - Day 11: Cosmic Expansion Part 1

use std::collections::HashSet;

pub(crate) fn solve(input: &str, expansion: usize) -> usize {
    // parse input into grid and find empty row and columns
    let mut empty_rows = Vec::new();
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            if !line.contains('#') {
                empty_rows.push(row_idx)
            }
            line.chars().collect()
        })
        .collect();
    let mut empty_cols = Vec::new();
    for col_idx in 0..grid[0].len() {
        if grid.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }

    // grid coordinates for all `#` galaxies
    let mut galaxies = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '#' {
                galaxies.push((row, col));
            }
        }
    }

    // expand universe by increasing galaxies coordinates
    for row_idx in empty_rows.iter().rev() {
        for galaxy_coord in &mut galaxies {
            if galaxy_coord.0 > *row_idx {
                galaxy_coord.0 += expansion - 1
            }
        }
    }
    for col_idx in empty_cols.iter().rev() {
        for galaxy_coord in &mut galaxies {
            if galaxy_coord.1 > *col_idx {
                galaxy_coord.1 += expansion - 1
            }
        }
    }

    // generate all possible pairs between two galaxies
    let combinations: HashSet<_> = (0..galaxies.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .collect();

    // compute sum of length between all paths
    combinations
        .iter()
        .map(|(i, j)| {
            galaxies[*i].0.abs_diff(galaxies[*j].0) + galaxies[*i].1.abs_diff(galaxies[*j].1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
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
        let answer = crate::part1::solve(example_input, 2);
        assert_eq!(answer, 374);
    }
}
