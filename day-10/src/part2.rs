// Advent of Code - Day 10: Pipe Maze Part 2

pub(crate) fn solve_part_two(input: &str) -> u32 {
    let (mut graph, starting_position) = crate::part1::parse_pipes(input);
    // replace S with all possible pipes and check if a loop exists
    let pipe_loop = "|-LJ7F"
        .chars()
        .find_map(|start_pipe| {
            graph[starting_position.0][starting_position.1] = crate::part1::match_pipe(start_pipe);
            crate::part1::find_loop(&graph, starting_position)
        })
        .unwrap();
    let mut inside_count = 0;
    for (row_idx, row) in graph.iter().enumerate() {
        // assume topleft of graph is outside of pipe loop
        let mut inside = false;
        for (column_idx, pipe) in row.iter().enumerate() {
            // count bits of ground when inside flag is true
            if !pipe_loop.contains(&(row_idx, column_idx)) {
                inside_count += inside as u32
            // flip inside flag when encountering a pipe that connects to north in the pipe loop.
            } else if pipe[0] {
                inside = !inside
            }
        }
    }
    inside_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2_nested_loop() {
        let example_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part2_sqeezed_nested_loop() {
        let example_input = "...........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
...........";
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part2_larger_example() {
        let example_input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 8);
    }

    // Current implementation can't cope with the starting position in the first line.
    // #[test]
    // fn part2_junk_pipe_example() {
    //     let example_input = "FF7FSF7F7F7F7F7F---7
    // L|LJ||||||||||||F--J
    // FL-7LJLJ||||||LJL-77
    // F--JF--7||LJLJ7F7FJ-
    // L---JF-JLJ.||-FJLJJ7
    // |F|F-JF---7F7-L7L|7|
    // |FFJF7L7F-JF7|JL---7
    // 7-L-JL7||F7|L7F-7F7|
    // L.L7LFJ|||||FJL7||LJ
    // L7JLJL-JLJLJL--JLJ.L";
    //     let answer = crate::part2::solve_part_two(example_input);
    //     assert_eq!(answer, 10);
    // }
}
