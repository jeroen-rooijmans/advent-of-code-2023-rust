// Advent of Code - Day 2: Cube Conundrum

const INPUT: &str = include_str!("./input.txt");

fn count_cubes(set: &str) -> [u32; 3] {
    let mut bag = [0; 3];
    let set = set.replace(',', "");
    let mut parts = set.split_whitespace();

    while let Some(quantity_part) = parts.next() {
        let quantity = quantity_part
            .parse::<u32>()
            .expect("Part should start with a parsable number");
        match parts.next() {
            Some("red") => bag[0] += quantity,
            Some("green") => bag[1] += quantity,
            Some("blue") => bag[2] += quantity,
            _ => unreachable!(),
        }
    }
    bag
}

fn solve_part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("Line should contain `: ` once.")
                .1
        })
        .map(|sets| {
            sets.split("; ")
                .map(count_cubes)
                .map(|bag| bag[0] <= 12 && bag[1] <= 13 && bag[2] <= 14)
                .collect::<Vec<bool>>()
        })
        .enumerate()
        .filter_map(|(idx, vec)| {
            if vec.iter().all(|&val| val) {
                Some(idx as u32 + 1)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("Line should contain `: ` once.")
                .1
        })
        .map(|sets| sets.split("; ").map(count_cubes).collect::<Vec<[u32; 3]>>())
        .map(|counts| {
            counts.iter().fold([0; 3], |acc, cubes| {
                acc.iter()
                    .zip(cubes.iter())
                    .map(|(&a, &b)| a.max(b))
                    .collect::<Vec<u32>>()
                    .try_into()
                    .unwrap()
            })
        })
        .map(|fewest_cubes| fewest_cubes.iter().product::<u32>())
        .sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\nSum of all possible games: {part_one_answer:?}");
    let part_two_answer: u32 = solve_part_two(INPUT);
    println!("Part two:\nSum of all calibration values: {part_two_answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 8);
    }

    #[test]
    fn part2() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 2286);
    }
}
