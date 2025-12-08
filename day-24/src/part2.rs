// Advent of Code - Day 24: Never Tell Me The Odds Part 2
use std::ops::{Add, Mul, Sub};

use num_bigint::BigInt;

type Coord = BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vector3d {
    x: Coord,
    y: Coord,
    z: Coord,
}
impl Add<&Vector3d> for &Vector3d {
    type Output = Vector3d;
    fn add(self, other: &Vector3d) -> Self::Output {
        Vector3d {
            x: self.x.clone() + &other.x,
            y: self.y.clone() + &other.y,
            z: self.z.clone() + &other.z,
        }
    }
}

impl Sub<&Vector3d> for &Vector3d {
    type Output = Vector3d;
    fn sub(self, other: &Vector3d) -> Self::Output {
        Vector3d {
            x: self.x.clone() - &other.x,
            y: self.y.clone() - &other.y,
            z: self.z.clone() - &other.z,
        }
    }
}

impl Mul<&Vector3d> for &Vector3d {
    type Output = Vector3d;
    fn mul(self, other: &Vector3d) -> Self::Output {
        Vector3d {
            x: self.x.clone() * &other.x,
            y: self.y.clone() * &other.y,
            z: self.z.clone() * &other.z,
        }
    }
}

impl Vector3d {
    /// Cross Product: A x B
    fn cross(&self, other: &Self) -> Self {
        Vector3d {
            x: self.y.clone() * &other.z - self.z.clone() * &other.y,
            y: self.z.clone() * &other.x - self.x.clone() * &other.z,
            z: self.x.clone() * &other.y - self.y.clone() * &other.x,
        }
    }

    /// Dot Product: A . B
    fn dot(&self, other: &Self) -> Coord {
        self.x.clone() * &other.x + self.y.clone() * &other.y + self.z.clone() * &other.z
    }
}

struct Hailstone {
    p: Vector3d,
    v: Vector3d,
}

// A_ij = (V_i - V_j) x (P_i - P_j)
fn get_a_vector(h1: &Hailstone, h2: &Hailstone) -> Vector3d {
    let dv = &h1.v - &h2.v;
    let dp = &h1.p - &h2.p;
    dv.cross(&dp)
}

// B_ij = (V_i - V_j) . (P_i x P_j)
fn get_b_scalar(h1: &Hailstone, h2: &Hailstone) -> Coord {
    let dv = &h1.v - &h2.v;
    let cross_p = h1.p.cross(&h2.p);
    dv.dot(&cross_p)
}

/// Calculates the determinant of 3d matrix
/// | a b c |
/// | d e f | = a(ei - fh) - b(di - fg) + c(dh - eg)
/// | g h i |
fn det3d(m: &[[Coord; 3]; 3]) -> Coord {
    m[0][0].clone() * (m[1][1].clone() * &m[2][2] - m[1][2].clone() * &m[2][1])
        - m[0][1].clone() * (m[1][0].clone() * &m[2][2] - m[1][2].clone() * &m[2][0])
        + m[0][2].clone() * (m[1][0].clone() * &m[2][1] - m[1][1].clone() * &m[2][0])
}

fn gaussian_elimination_solver(m: &[[Coord; 3]; 3], b: &[Coord; 3]) -> Vector3d {
    let det_m = det3d(m);
    assert!(
        det_m != BigInt::from(0),
        "Cannot solve singular matrix (det=0)"
    );

    // Solve for Px
    let mut mx = m.clone();
    mx[0][0].clone_from(&b[0]);
    mx[1][0].clone_from(&b[1]);
    mx[2][0].clone_from(&b[2]);
    let det_mx = det3d(&mx);

    // Solve for Py
    let mut my = m.clone();
    my[0][1].clone_from(&b[0]);
    my[1][1].clone_from(&b[1]);
    my[2][1].clone_from(&b[2]);
    let det_my = det3d(&my);

    // Solve for Pz
    let mut mz = m.clone();
    mz[0][2].clone_from(&b[0]);
    mz[1][2].clone_from(&b[1]);
    mz[2][2].clone_from(&b[2]);
    let det_mz = det3d(&mz);

    // Divide by original determinant to get solution
    Vector3d {
        x: det_mx / &det_m,
        y: det_my / &det_m,
        z: det_mz / &det_m,
    }
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('@').collect();
            // Function to parse the 3 coordinates (Px, Py, Pz) or (Vx, Vy, Vz)
            let parse_coords = |s: &str| -> Option<Vector3d> {
                let coords: Vec<i128> = s
                    .split(',')
                    .map(|c| c.trim().parse::<i128>())
                    .filter_map(Result::ok)
                    .collect();

                if coords.len() == 3 {
                    Some(Vector3d {
                        x: coords[0].into(),
                        y: coords[1].into(),
                        z: coords[2].into(),
                    })
                } else {
                    None
                }
            };

            let p_vec = parse_coords(parts[0])?;
            let v_vec = parse_coords(parts[1])?;

            Some(Hailstone { p: p_vec, v: v_vec })
        })
        .collect()
}

pub(crate) fn solve(input: &str) -> BigInt {
    let hailstones = parse_input(input);
    // use first 3 hailstones to from pairs (0,1), (0,2) and (1,2)
    let hailstone_0 = &hailstones[0];
    let hailstone_1 = &hailstones[1];
    let hailstone_2 = &hailstones[2];

    let a01 = get_a_vector(hailstone_0, hailstone_1);
    let b01 = get_b_scalar(hailstone_0, hailstone_1);
    let a02 = get_a_vector(hailstone_0, hailstone_2);
    let b02 = get_b_scalar(hailstone_0, hailstone_2);
    let a12 = get_a_vector(hailstone_1, hailstone_2);
    let b12 = get_b_scalar(hailstone_1, hailstone_2);

    // construct 3x3 coefficient matrix M
    let m = [
        [a01.x, a01.y, a01.z],
        [a02.x, a02.y, a02.z],
        [a12.x, a12.y, a12.z],
    ];
    // 3x1 constant vector B
    let b = [b01, b02, b12];

    // Solve for rock initial position P
    let p = gaussian_elimination_solver(&m, &b);

    // sum coordinates of rock's initial position
    p.x + p.y + p.z
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, num_bigint::BigInt::from(47));
    }
}
