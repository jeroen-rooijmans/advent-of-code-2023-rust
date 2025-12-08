// Advent of Code - Day 24: Never Tell Me The Odds Part 1
//
use std::{ops::RangeInclusive, str::FromStr};

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug)]
struct ParseHailstoneError;

impl FromStr for Hailstone {
    type Err = ParseHailstoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('@', ",").replace(' ', "");
        let mut parts = s.split(',');
        let px = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        let py = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        let pz = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        let vx = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        let vy = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        let vz = parts
            .next()
            .unwrap()
            .parse::<f64>()
            .map_err(|_| ParseHailstoneError)?;
        Ok(Hailstone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        })
    }
}

impl Hailstone {
    fn intersect(self, other: &Self) -> Option<(f64, f64)> {
        let det = (self.vy * other.vx) - (self.vx * other.vy);
        if det == 0.0 {
            return None;
        }

        let t1 = (other.vx * (other.py - self.py) - other.vy * (other.px - self.px)) / det;
        let t2 = (self.vx * (other.py - self.py) - self.vy * (other.px - self.px)) / det;
        if [t1, t2].iter().any(|t| t.is_sign_negative()) {
            return None;
        }

        let x_intersect = self.px + t1 * self.vx;
        let y_intersect = self.py + t1 * self.vy;

        Some((x_intersect, y_intersect))
    }
}

fn combinations<T>(vec: &[T]) -> Vec<(usize, usize)>
where
    T: Clone,
{
    let mut combinations = Vec::new();
    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            combinations.push((i, j));
        }
    }
    combinations
}

pub(crate) fn solve(input: &str, test_area: RangeInclusive<f64>) -> usize {
    let hailstones = input
        .lines()
        .map(|l| Hailstone::from_str(l).unwrap())
        .collect::<Vec<_>>();
    combinations(&hailstones)
        .iter()
        .filter_map(|&(f, s)| hailstones[f].intersect(&hailstones[s]))
        .filter(|(x, y)| test_area.contains(x) && test_area.contains(y))
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let answer = crate::part1::solve(example_input, 7.0..=27.0);
        assert_eq!(answer, 2);
    }
}
