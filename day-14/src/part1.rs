// Advent of Code - Day 14: Parabolic Reflector Dish Part 1

pub(crate) fn solve(input: &str) -> usize {
    let mut platform: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
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
    fn part1() {
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
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 136);
    }
}
