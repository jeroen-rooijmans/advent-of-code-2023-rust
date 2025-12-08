// Advent of Code - Day 20: Step Counter Part 2

use std::collections::{HashMap, VecDeque};

fn find_start_pos(grid: &[Vec<char>]) -> (isize, isize) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                return (r as isize, c as isize);
            }
        }
    }
    panic!("Input grid must contain a starting position 'S'");
}

/// Breadth-first search to calculate shortest path to every reachable plot
/// on an infinite repeating grid
fn bfs(
    grid: &[Vec<char>],
    start_pos: (isize, isize),
    max_steps: usize,
) -> HashMap<(isize, isize), usize> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut distances = HashMap::from([(start_pos, 0)]);
    let mut queue = VecDeque::from([start_pos]);

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        let dist = *distances.get(&(r, c)).unwrap();
        if dist >= max_steps {
            continue;
        }

        for (dr, dc) in directions {
            let nr = r + dr;
            let nc = c + dc;

            // Use modulo to wrap coordinates to the original grid
            let grid_r = nr.rem_euclid(rows) as usize;
            let grid_c = nc.rem_euclid(cols) as usize;

            if grid[grid_r][grid_c] != '#'
                && let std::collections::hash_map::Entry::Vacant(e) = distances.entry((nr, nc))
            {
                e.insert(dist + 1);
                queue.push_back((nr, nc));
            }
        }
    }
    distances
}

pub(crate) fn solve(input: &str) -> usize {
    // key observations:
    // all edges are reachable from starting position
    // border itself is clear
    // input grid has diamond pattern that will be expended into infinity
    // (i.e. original grid contains 1 diamond in the middle, and 4 quarter diamonds in each corner.
    //
    // Therefore: the number of reachable plots after n steps on the infinite grid
    // is a quadratic formula of the numbers of full grids traversed plus any remaining steps on the last grid.
    // step count S = n * grid_size + remainder_after_traversing_whole_grids
    // remainder = S % 131 = 65
    // 26501365 = n * 131 + 65, solve for n gives 202300

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_size = grid.len(); // grid is assumed to be square for this solution
    let steps = 26501365;
    let remaining_steps = steps % grid_size; // 65
    let start_pos = find_start_pos(&grid);

    // To interpolate a quadratic curve: f(k) = A k^2 + B k + C, we'll compute 3 data points
    // (k=0, y0), (k=1, y1), (k=2, y2)
    // f(0): C => y0 = C
    // f(1): A + B + C => y1 = A + B + y0 => y1 - y0 = A + B
    // f(2): 4A + 2B + C => y2 = 4A + 2B + y0 => y2 - y0 = 4A + 2B)

    // Run BFS for max numbers of steps needed to interpolate (2 grid traversals + remaining steps)
    let distances = bfs(&grid, start_pos, 2 * grid_size + remaining_steps);

    // (0, y0): data point for remaining_steps
    let y0_steps = remaining_steps;
    let parity = y0_steps % 2;
    let y0 = distances
        .values()
        .filter(|&dist| dist <= &y0_steps && dist % 2 == parity)
        .count() as isize;

    // (1, Y_1): data point for single full grid traversal plus remaining_steps
    let y1_steps = grid_size + remaining_steps;
    let parity = y1_steps % 2;
    let y1 = distances
        .values()
        .filter(|&dist| dist <= &y1_steps && dist % 2 == parity)
        .count() as isize;

    // (2, Y_1): data point for two full grid traversals plus remaining_steps
    let y2_steps = 2 * grid_size + remaining_steps;
    let parity = y2_steps % 2;
    let y2 = distances
        .values()
        .filter(|&dist| dist <= &y2_steps && dist % 2 == parity)
        .count() as isize;

    // Solve for A
    // A + B = y1 - y0
    // (1) 2A + 2B = 2y1 - 2y0
    //
    // (2) 4A + 2B = y2 - y0
    //
    // (2) - (1)
    // (4A + 2B) - (2A + 2B) = (y2 - y0) - (2y1 - 2y0)
    // 2A = y2 - 2y1 + y0
    let two_a = y2 - 2 * y1 + y0;
    let a = two_a / 2;

    // Solving for B
    // A + B = y1 - y0
    // B = y1 - y0 - A
    let b = y1 - y0 - a;

    // Solve for C
    // C = y0
    let c = y0;

    // Calculate f(n) = A*n^2 + B*n + C
    // steps = n * grid_size + remaining_steps
    // n = (steps - remaining_steps) / grid_size
    let n = ((steps - remaining_steps) / grid_size) as isize;
    (a * (n * n) + b * n + c) as usize
}
