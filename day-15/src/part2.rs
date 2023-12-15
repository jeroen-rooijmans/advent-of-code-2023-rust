// Advent of Code - Day 15: Lens Library Part 2

use crate::part1::hash;

pub(crate) fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for step in input.split(',') {
        if step.contains('-') {
            let label = &step[..step.len() - 1];
            boxes[hash(label)].retain(|&(l, _)| l != label);
        } else {
            let (label, focal_length) = {
                let (l, fl) = step.split_once('=').unwrap();
                (l, fl.trim().parse::<usize>().unwrap())
            };
            if let Some((_, fl)) = boxes[hash(label)].iter_mut().find(|(l, _)| l == &label) {
                *fl = focal_length;
            } else {
                boxes[hash(label)].push((label, focal_length));
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(move |(lens_idx, (_, focal_length))| {
                    (box_idx + 1) * (lens_idx + 1) * focal_length
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 145);
    }
}
