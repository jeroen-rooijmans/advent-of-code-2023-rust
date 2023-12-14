// Advent of Code - Day 14: Parabolic Reflector Dish Part 2

use std::collections::HashMap;

fn roll_north(platform: &mut Vec<Vec<u8>>) {
    let mut finished = false;
    while !finished {
        finished = true;
        for row_idx in 0..platform.len() - 1 {
            for col_idx in 0..platform[0].len() {
                if platform[row_idx + 1][col_idx] == b'O' && platform[row_idx][col_idx] == b'.' {
                    platform[row_idx][col_idx] = b'O';
                    platform[row_idx + 1][col_idx] = b'.';
                    finished = false;
                }
            }
        }
    }
}

fn rotate_ccw(platform: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotated_platform = vec![vec![0; platform.len()]; platform[0].len()];
    for row_idx in 0..rotated_platform.len() {
        for col_idx in 0..rotated_platform[0].len() {
            rotated_platform[col_idx][platform.len() - 1 - row_idx] = platform[row_idx][col_idx];
        }
    }
    rotated_platform
}

pub(crate) fn solve(input: &str) -> usize {
    let mut platform: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut cache = HashMap::new();
    for itr in 1..1_000_000_000 {
        for _ in 0..4 {
            roll_north(&mut platform);
            platform = rotate_ccw(&platform);
        }
        if let Some(cached_itr) = cache.insert(platform.clone(), itr) {
            if (1_000_000_000 - itr) % (itr - cached_itr) == 0 {
                break;
            }
        }
    }
    let load_factor = platform.len();
    platform
        .iter()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&i| i == &b'O').count() * (load_factor - idx))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 64);
    }
}
