// Advent of Code - Day 15: The Floor Will Be Lava Part 1

use std::collections::{HashSet, VecDeque};

fn starting_direction(grid: &[Vec<char>], position: (usize, usize), direction: char) -> char {
    match (grid[position.0][position.1], direction) {
        ('.', _) | ('-', 'l' | 'r') | ('|', 'u' | 'd') => direction,
        ('/', 'u') | ('\\', 'd') => 'r',
        ('/', 'r') | ('\\', 'l') => 'u',
        ('/', 'd') | ('\\', 'u') => 'l',
        ('/', 'l') | ('\\', 'r') => 'd',
        ('-', _) if position.1 == 0 => 'r',
        ('-', _) if position.1 != 0 => 'l',
        ('|', _) if position.0 == 0 => 'd',
        ('|', _) if position.0 != 0 => 'u',
        _ => unreachable!(),
    }
}

pub(crate) fn count_energised_tiles(
    grid: &Vec<Vec<char>>,
    initial_position: (usize, usize),
    initial_direction: char,
) -> usize {
    let mut energised = HashSet::new();
    let mut seen = HashSet::new();
    let mut beams = VecDeque::from(vec![(
        initial_position,
        starting_direction(grid, initial_position, initial_direction),
    )]);

    while let Some((beam_position, beam_direction)) = beams.pop_front() {
        if seen.contains(&(beam_position, beam_direction)) {
            continue;
        }
        energised.insert(beam_position);
        seen.insert((beam_position, beam_direction));
        let new_beams: Vec<_> = match (beam_position, beam_direction) {
            ((row_idx, col_idx), 'l') if col_idx > 0 => vec![(row_idx, col_idx - 1, 'l')],
            ((row_idx, col_idx), 'r') if col_idx < grid[0].len() - 1 => {
                vec![(row_idx, col_idx + 1, 'r')]
            }
            ((row_idx, col_idx), 'u') if row_idx > 0 => vec![(row_idx - 1, col_idx, 'u')],
            ((row_idx, col_idx), 'd') if row_idx < grid.len() - 1 => {
                vec![(row_idx + 1, col_idx, 'd')]
            }
            _ => vec![],
        }
        .into_iter()
        .flat_map(
            |(new_row, new_col, dir)| match (grid[new_row][new_col], dir) {
                ('.', _) | ('-', 'l' | 'r') | ('|', 'u' | 'd') => vec![(new_row, new_col, dir)],
                ('/', 'u') | ('\\', 'd') => vec![(new_row, new_col, 'r')],
                ('/', 'r') | ('\\', 'l') => vec![(new_row, new_col, 'u')],
                ('/', 'd') | ('\\', 'u') => vec![(new_row, new_col, 'l')],
                ('/', 'l') | ('\\', 'r') => vec![(new_row, new_col, 'd')],
                ('-', _) => vec![(new_row, new_col, 'l'), (new_row, new_col, 'r')],
                ('|', _) => vec![(new_row, new_col, 'u'), (new_row, new_col, 'd')],
                _ => vec![],
            },
        )
        .collect();

        beams.extend(
            new_beams
                .into_iter()
                .map(|(row_idx, col_idx, direction)| ((row_idx, col_idx), direction)),
        );
    }
    energised.len()
}

pub(crate) fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    count_energised_tiles(&grid, (0, 0), 'r')
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = include_str!("./example.txt");
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 46);
    }

    #[test]
    fn part1_out_of_bounds_1() {
        let example_input = "......";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn part1_out_of_bounds_2() {
        let example_input = "..../.";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part1_out_of_bounds_3() {
        let example_input = "....\\.";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part1_out_of_bounds_4() {
        let example_input = "/";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn part1_out_of_bounds_5() {
        let example_input = "\\
.
.
.
.";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 5);
    }
}
