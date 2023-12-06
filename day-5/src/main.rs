// Advent of Code - Day 5: If You Give A Seed A Fertilizer

const INPUT: &str = include_str!("./input.txt");

struct Mapping {
    maps: Vec<Map>,
}

impl Mapping {
    fn new() -> Self {
        Mapping { maps: vec![] }
    }
    fn add_map(&mut self, map: Map) {
        self.maps.push(map);
        self.maps.sort_by_key(|r| r.range.start)
    }

    fn apply(&self, val: u64) -> u64 {
        for map in &self.maps {
            if map.range.contains(&val) {
                return (val as i64 + map.offset) as u64;
            }
        }
        val
    }
}

#[derive(Debug)]
struct Map {
    range: std::ops::Range<u64>,
    offset: i64,
}

impl Map {
    fn from_almanac(dest: u64, src: u64, len: u64) -> Self {
        Map {
            range: src..src + len,
            offset: dest as i64 - src as i64,
        }
    }
}

fn solve_part_one(input: &str) -> u64 {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let mut mappings: Vec<Mapping> = Vec::new();
    let mut mapping = Mapping::new();
    for line in lines {
        if line.contains("map") {
            if !mapping.maps.is_empty() {
                mappings.push(mapping);
                mapping = Mapping::new();
            }
            continue;
        }

        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        mapping.add_map(Map::from_almanac(nums[0], nums[1], nums[2]));
    }
    if !mapping.maps.is_empty() {
        mappings.push(mapping);
    }

    let mut lowest_location: u64 = u64::MAX;
    for seed in seeds {
        let mut cur = seed;
        for mapping in &mappings {
            cur = mapping.apply(cur)
        }
        lowest_location = lowest_location.min(cur);
    }
    lowest_location
}

fn solve_part_two(input: &str) -> u64 {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let seed_ranges = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|range| range[0]..range[0] + range[1])
        .collect::<Vec<std::ops::Range<u64>>>();

    let mut mappings: Vec<Mapping> = Vec::new();
    let mut mapping = Mapping::new();
    for line in lines {
        if line.contains("map") {
            if !mapping.maps.is_empty() {
                mappings.push(mapping);
                mapping = Mapping::new();
            }
            continue;
        }

        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        mapping.add_map(Map::from_almanac(nums[0], nums[1], nums[2]));
    }
    if !mapping.maps.is_empty() {
        mappings.push(mapping);
    }

    let mut lowest_location = u64::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let mut cur = seed;
            for mapping in &mappings {
                cur = mapping.apply(cur)
            }
            lowest_location = lowest_location.min(cur);
        }
    }
    lowest_location
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one: {part_one_answer:?}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two: {part_two_answer:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 35);
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 46);
    }
}
