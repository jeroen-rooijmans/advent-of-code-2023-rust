// Advent of Code - Day 10: Pipe Maze Part 1

use std::collections::HashSet;

pub(crate) fn match_pipe(pipe: char) -> [bool; 4] {
    match pipe {
        //     [north, east, south, west]
        '|' => [true, false, true, false],
        '-' => [false, true, false, true],
        'L' => [true, true, false, false],
        'J' => [true, false, false, true],
        '7' => [false, false, true, true],
        'F' => [false, true, true, false],
        _ => [false, false, false, false],
    }
}

pub(crate) fn parse_pipes(input: &str) -> (Vec<Vec<[bool; 4]>>, (usize, usize)) {
    let mut starting_position = (0, 0);
    let graph = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(column_idx, pipe)| {
                    if pipe == 'S' {
                        starting_position = (row_idx, column_idx);
                    }
                    match_pipe(pipe)
                })
                .collect::<Vec<[bool; 4]>>()
        })
        .collect::<Vec<_>>();
    (graph, starting_position)
}

pub(crate) fn find_loop(
    graph: &[Vec<[bool; 4]>],
    starting_position: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let (mut row_idx, mut column_idx) = starting_position;
    let mut pipe_loop = HashSet::new();
    let mut direction = graph[row_idx][column_idx]
        .iter()
        .position(|&dir| dir)
        .unwrap();
    loop {
        if !pipe_loop.insert((row_idx, column_idx)) {
            break Some(pipe_loop);
        }
        let previous_pipe = match direction {
            // north => headed south
            0 => {
                row_idx -= 1;
                2
            }
            // east => headed west
            1 => {
                column_idx += 1;
                3
            }
            // south => headed north
            2 => {
                row_idx += 1;
                0
            }
            // west => headed east
            3 => {
                column_idx -= 1;
                1
            }
            _ => unreachable!(),
        };
        if !graph[row_idx][column_idx][previous_pipe] {
            break None;
        }
        direction = (0..4)
            .find(|&i| i != previous_pipe && graph[row_idx][column_idx][i])
            .unwrap();
    }
}

pub(crate) fn solve_part_one(input: &str) -> u32 {
    let (mut graph, starting_position) = parse_pipes(input);
    // replace S with all possible pipes and check if a loop exists
    let pipe_loop = "|-LJ7F"
        .chars()
        .find_map(|start_pipe| {
            graph[starting_position.0][starting_position.1] = match_pipe(start_pipe);
            find_loop(&graph, starting_position)
        })
        .unwrap();
    // farthest position is halfway down the loop
    u32::try_from(pipe_loop.len()).unwrap() / 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_square_loop() {
        let example_input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part1_square_loop_extra_pipes() {
        let example_input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part1_complex_loop() {
        let example_input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 8);
    }

    #[test]
    fn part1_complex_loop_extra_pipes() {
        let example_input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let answer = crate::part1::solve_part_one(example_input);
        assert_eq!(answer, 8);
    }
}
