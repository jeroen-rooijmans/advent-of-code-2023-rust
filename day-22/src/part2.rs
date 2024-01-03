// Advent of Code - Day 22: Sand Slabs Part 2

pub(crate) fn solve(input: &str) -> usize {
    let mut bricks = input
        .lines()
        .map(|l| {
            let mut coord_iter = l
                .split('~')
                .flat_map(|raw_coords| raw_coords.split(',').flat_map(|c| c.parse::<usize>()));
            [
                coord_iter.next().unwrap(),
                coord_iter.next().unwrap(),
                coord_iter.next().unwrap(),
                coord_iter.next().unwrap(),
                coord_iter.next().unwrap(),
                coord_iter.next().unwrap(),
            ]
        })
        .collect::<Vec<[usize; 6]>>();

    // sort bricks by first z coordinate (this is always the lower of the two z coordinates).
    bricks.sort_by_key(|&[_, _, z, ..]| z);

    let collapsed = crate::part1::collapse(bricks);
    let mut fallen_bricks = 0;
    for idx in 0..collapsed.len() {
        let mut disintegrated = collapsed.clone();
        let brick = disintegrated.remove(idx);
        disintegrated = crate::part1::collapse(disintegrated);
        disintegrated.insert(idx, brick);
        fallen_bricks += collapsed
            .iter()
            .zip(disintegrated.iter())
            .filter(|&(original_brick, fallen_brick)| original_brick != fallen_brick)
            .count();
    }
    fallen_bricks
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 7);
    }
}
