// Advent of Code - Day 22: Sand Slabs Part 1

fn resting_on(brick: &[usize; 6]) -> Vec<(usize, usize)> {
    (brick[0]..=brick[3])
        .flat_map(|x| (brick[1]..=brick[4]).map(move |y| (x, y)))
        .collect()
}

pub(crate) fn collapse(bricks: Vec<[usize; 6]>) -> Vec<[usize; 6]> {
    let mut floor = [[0; 10]; 10];
    let mut collapsed = Vec::new();
    for mut brick in bricks {
        let resting_coords = resting_on(&brick);
        let current_floor = resting_coords
            .iter()
            .map(|&(x, y)| floor[x][y])
            .max()
            .unwrap();
        if brick[2] > current_floor + 1 {
            let height = brick[5] - brick[2];
            brick[2] = current_floor + 1;
            brick[5] = brick[2] + height;
        };
        collapsed.push(brick);
        resting_coords.iter().for_each(|&(x, y)| {
            floor[x][y] = brick[5];
        });
    }
    collapsed
}

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

    let collapsed = collapse(bricks);
    let mut safe = 0;
    for idx in 0..collapsed.len() {
        let mut disintegrated = collapsed.clone();
        disintegrated.remove(idx);
        if disintegrated == collapse(disintegrated.clone()) {
            safe += 1;
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 5);
    }
}
