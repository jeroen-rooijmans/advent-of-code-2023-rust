// Advent of Code - Day 20: Step Counter Part 1

use std::collections::{HashMap, VecDeque};

fn find_start_pos(grid: &[Vec<char>]) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                return (r, c);
            }
        }
    }
    panic!("Input grid must contain a starting position 'S'");
}

/// Breadth-first search to calculate shortest path to every reachable plot
fn bfs(
    grid: &[Vec<char>],
    start_pos: (usize, usize),
    max_steps: usize,
) -> HashMap<(usize, usize), usize> {
    let mut distances = HashMap::from([(start_pos, 0)]);
    let mut queue = VecDeque::from([start_pos]);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        let dist = *distances.get(&(r, c)).unwrap();
        if dist >= max_steps {
            continue;
        }

        for (dr, dc) in directions {
            // compute new positions, while checking we're still on the board.
            let nr = usize::try_from(dr + (r as isize)).unwrap();
            let nc = usize::try_from(dc + (c as isize)).unwrap();

            if grid[nr][nc] != '#'
                && let std::collections::hash_map::Entry::Vacant(e) = distances.entry((nr, nc))
            {
                e.insert(dist + 1);
                queue.push_back((nr, nc));
            }
        }
    }

    distances
}

pub(crate) fn solve(input: &str, steps: usize) -> usize {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_pos = find_start_pos(&grid);
    let distances = bfs(&grid, start_pos, steps);
    // filter out plots that can be reached in `parity of steps` number of steps
    let parity = steps % 2;
    distances
        .values()
        .filter(|&dist| dist % 2 == parity)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let answer = crate::part1::solve(example_input, 6);
        assert_eq!(answer, 16);
    }
}
