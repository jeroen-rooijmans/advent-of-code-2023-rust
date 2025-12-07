// Advent of Code - Day 23: A Long Walk Part 1

use std::collections::{HashMap, HashSet};

fn exhaustive_depth_first_search(
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: &(usize, usize),
    goal: &(usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    fn dfs_recursive(
        graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
        current: &(usize, usize),
        goal: &(usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        path: &mut Vec<(usize, usize)>,
        all_paths: &mut Vec<Vec<(usize, usize)>>,
    ) {
        visited.insert(*current);

        if current == goal {
            // Goal reached, add the current path to the result
            all_paths.push(path.clone());
            visited.remove(current); // Backtrack
            return;
        }

        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    path.push(*neighbor);
                    dfs_recursive(graph, neighbor, goal, visited, path, all_paths);
                    path.pop();
                }
            }
        }

        visited.remove(current); // Backtrack
    }

    let mut visited = HashSet::new();
    let mut path = vec![*start];
    let mut all_paths = Vec::new();

    dfs_recursive(graph, start, goal, &mut visited, &mut path, &mut all_paths);

    all_paths
}

fn find_neighbours(
    grid: &[Vec<char>],
    row_idx: usize,
    col_idx: usize,
    around: &[(isize, isize)],
) -> Vec<(usize, usize)> {
    around
        .iter()
        .map(|&(d_row, d_col)| {
            (
                (row_idx as isize + d_row) as usize,
                (col_idx as isize + d_col) as usize,
            )
        })
        .filter(|&(r, c)| grid[r][c] != '#')
        .collect::<Vec<_>>()
}

pub(crate) fn solve(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    grid.insert(0, vec!['#'; grid[0].len()]);
    grid.insert(grid.len(), vec!['#'; grid[0].len()]);
    let mut graph = HashMap::new();
    for row_idx in 1..grid.len() - 1 {
        for col_idx in 1..grid[0].len() - 1 {
            let neighbours = match grid[row_idx][col_idx] {
                '#' => continue,
                '.' => {
                    find_neighbours(&grid, row_idx, col_idx, &[(-1, 0), (0, 1), (1, 0), (0, -1)])
                }
                '>' => find_neighbours(&grid, row_idx, col_idx, &[(0, 1)]),
                'v' => find_neighbours(&grid, row_idx, col_idx, &[(1, 0)]),
                _ => unreachable!(),
            };
            graph.insert((row_idx, col_idx), neighbours);
        }
    }
    let start = (1, 1);
    let goal = (grid.len() - 2, grid[0].len() - 2);
    let paths = exhaustive_depth_first_search(&graph, &start, &goal);
    paths.iter().map(|p| p.len() - 1).max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 94);
    }
}
