// Advent of Code - Day 18: Lavaduct Lagoon Part 1

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction char: {}", c),
        }
    }
}

/// Represents a single instruction from the input line.
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    // Parse line: "R 6 (#70c710)" -> Instruction { Right, 6 }
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let dir_char = parts
            .next()
            .expect("Missing direction")
            .chars()
            .next()
            .unwrap();
        let dist_str = parts.next().expect("Missing distance");
        Self {
            direction: Direction::from(dir_char),
            distance: dist_str.parse().expect("Invalid number"),
        }
    }
}

pub(crate) fn solve(input: &str) -> usize {
    let dig_plan = input.lines().map(Instruction::parse);

    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;

    // Shoelace area accumulator
    let mut area_sum: i64 = 0;
    // The perimeter is needed for Pick's Theorem adjustment
    let mut perimeter: i64 = 0;

    for instr in dig_plan {
        let (dx, dy) = match instr.direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        };

        let step_x = dx * instr.distance;
        let step_y = dy * instr.distance;

        let next_x = current_x + step_x;
        let next_y = current_y + step_y;

        // Accumulate Perimeter
        perimeter += instr.distance;

        // Accumulate Shoelace Area
        // Formula component: (x_i * y_{i+1}) - (x_{i+1} * y_i)
        area_sum += (current_x * next_y) - (next_x * current_y);

        current_x = next_x;
        current_y = next_y;
    }

    // Standard Polygon Area = |sum| / 2
    let polygon_area = area_sum.abs() / 2;

    // Pick's Theorem logic:
    // The Shoelace formula gives the area of the shape defined by the *center* of the points.
    // However, AoC grid problems treat points as 1x1 blocks.
    // We need to include the "outer half" of the border blocks.
    // Total Area = PolygonArea + (Perimeter / 2) + 1
    let total_area = polygon_area + (perimeter / 2) + 1;

    total_area as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 62);
    }
}
