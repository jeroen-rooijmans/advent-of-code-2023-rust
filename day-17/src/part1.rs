use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub(crate) fn solve(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    // Heatloss array stores optimal heatloss values for blocks
    // shape: [row][col][axis]
    // axis: 0 => arrived here with a vertical move
    // axis: 1 => arrived here with a horizontal move
    let mut heatlosses = vec![vec![[u32::MAX; 2]; cols]; rows];

    // State: (HeatLoss, Row, Col, Axis)
    let mut pq = BinaryHeap::new();

    // Crucible starts at (0,0) with 0 heat loss
    heatlosses[0][0] = [0, 0];
    // Push both axis states to allow for next step in east or south direction
    pq.push(Reverse((0, 0, 0, 0)));
    pq.push(Reverse((0, 0, 0, 1)));

    while let Some(Reverse((heatloss, r, c, axis))) = pq.pop() {
        // Skip if we've found a better route here
        if heatloss > heatlosses[r][c][axis] {
            continue;
        }

        // Check if crucible arrived at target
        if (r, c) == (rows - 1, cols - 1) {
            return heatloss;
        }

        // Move 1 to 3 blocks in "other" direction
        for sign in [-1isize, 1] {
            let mut current_heatloss = heatloss;
            for i in 1..=3 {
                let (nr, nc) = if axis == 0 {
                    // horizontal move
                    (r as isize, c as isize + sign * i)
                } else {
                    // vertical move
                    (r as isize + sign * i, c as isize)
                };

                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    break;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                // Add heatloss of new block
                current_heatloss += grid[nr][nc];

                let new_axis = 1 - axis;
                // Update heatlosses and state if current heatloss is best so far
                if current_heatloss < heatlosses[nr][nc][new_axis] {
                    heatlosses[nr][nc][new_axis] = current_heatloss;
                    pq.push(Reverse((current_heatloss, nr, nc, new_axis)));
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 102);
    }
}
