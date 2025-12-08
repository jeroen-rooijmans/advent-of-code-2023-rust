// Advent of Code - Day 23: A Long Walk Part 2

use std::collections::{HashMap, HashSet, VecDeque};

type GridGraph = HashMap<(usize, usize), Vec<(usize, usize)>>;
type ContractedEdge = ((usize, usize), usize);
type ContractedGraph = HashMap<(usize, usize), Vec<ContractedEdge>>;

fn find_neighbours(grid: &[Vec<char>], row_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(|&(dr, dc)| {
            let next_row = row_idx as isize + dr;
            let next_col = col_idx as isize + dc;
            if next_row < 0
                || next_row >= grid.len() as isize
                || next_col < 0
                || next_col >= grid[0].len() as isize
            {
                return None;
            }
            let r = next_row as usize;
            let c = next_col as usize;
            if grid[r][c] != '#' {
                Some((r, c))
            } else {
                None
            }
        })
        .collect()
}

/// Build smaller weighted graph
/// Nodes are junctions (start, goal and places with >= 3 neighbours)
/// Edge weight is the length of the shortest path between the node junctions
fn contract_graph(
    grid_graph: &GridGraph,
    start: (usize, usize),
    goal: (usize, usize),
) -> ContractedGraph {
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    nodes.insert(start);
    nodes.insert(goal);

    for (&coord, neighbours) in grid_graph.iter() {
        if neighbours.len() >= 3 {
            nodes.insert(coord);
        }
    }

    let mut contracted_graph = HashMap::new();
    for &node in nodes.iter() {
        let mut edges = Vec::new();
        // BFS for shortest paths to other nodes
        let mut queue = VecDeque::from([(node, 0)]);
        let mut visited = HashSet::from([node]);

        while let Some((current_node, distance)) = queue.pop_front() {
            if current_node != node && nodes.contains(&current_node) {
                // found an edge in contracted graph
                edges.push((current_node, distance));
                continue;
            }

            if let Some(neighbours) = grid_graph.get(&current_node) {
                for &neighbour in neighbours {
                    if !visited.contains(&neighbour) {
                        visited.insert(neighbour);
                        queue.push_back((neighbour, distance + 1));
                    }
                }
            }
        }
        contracted_graph.insert(node, edges);
    }
    contracted_graph
}

/// Finds the maximum path length for all paths from start to goal
fn exhaustive_weighted_depth_first_search(
    graph: &ContractedGraph,
    start: &(usize, usize),
    goal: &(usize, usize),
) -> usize {
    let mut max_length = 0;

    fn dfs_recursive(
        graph: &ContractedGraph,
        current: &(usize, usize),
        goal: &(usize, usize),
        visited: &mut HashSet<(usize, usize)>,
        current_length: usize,
        max_length: &mut usize,
    ) {
        if current == goal {
            *max_length = (*max_length).max(current_length);
            return;
        }

        visited.insert(*current);

        if let Some(neighbours) = graph.get(current) {
            for &(neighbour, weight) in neighbours {
                if !visited.contains(&neighbour) {
                    dfs_recursive(
                        graph,
                        &neighbour,
                        goal,
                        visited,
                        current_length + weight,
                        max_length,
                    );
                }
            }
        }
        visited.remove(current); // backtrack
    }

    let mut visisted = HashSet::new();
    dfs_recursive(graph, start, goal, &mut visisted, 0, &mut max_length);
    max_length
}

pub(crate) fn solve(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    // treat all slopes as paths
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if *c == 'v' || *c == '>' {
                *c = '.'
            }
        }
    }

    // build graph of all paths
    let mut grid_graph = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                let neighbours = find_neighbours(&grid, r, c);
                grid_graph.insert((r, c), neighbours);
            }
        }
    }

    let start = (0, 1);
    let goal = (grid.len() - 1, grid[0].len() - 2);

    let contracted_graph = contract_graph(&grid_graph, start, goal);
    exhaustive_weighted_depth_first_search(&contracted_graph, &start, &goal)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
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
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 154);
    }
}
