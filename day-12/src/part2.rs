// Advent of Code - Day 12: Hot Springs Part 2

use std::collections::HashMap;

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if let (Some(folded_springs), Some(nums)) = (parts.first(), parts.last()) {
        let mut springs = String::from(*folded_springs);
        for _ in 0..4 {
            springs.push('?');
            springs += folded_springs;
        }
        let nums = nums
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<usize>>();
        (
            springs,
            nums.iter().cloned().cycle().take(nums.len() * 5).collect(),
        )
    } else {
        panic!("Invalid input!");
    }
}

#[allow(clippy::type_complexity)]
fn count_arrangements<'a>(
    springs: &'a [u8],
    in_group: Option<usize>,
    nums: &'a [usize],
    cache: &mut HashMap<(&'a [u8], Option<usize>, &'a [usize]), usize>,
) -> usize {
    if springs.is_empty() {
        return match in_group {
            Some(n) if nums == [n] => 1,
            None if nums.is_empty() => 1,
            _ => 0,
        };
    }
    if springs[0] == b'?' {
        if let Some(result) = cache.get(&(springs, in_group, nums)) {
            return *result;
        }
    }
    let result = match (springs[0], in_group, nums) {
        (b'.', None, _) | (b'?', None, []) => count_arrangements(&springs[1..], None, nums, cache),
        (b'.' | b'?', Some(n), [e, ..]) if n == *e => {
            count_arrangements(&springs[1..], None, &nums[1..], cache)
        }
        (b'#' | b'?', Some(n), [e, ..]) if n < *e => {
            count_arrangements(&springs[1..], Some(n + 1), nums, cache)
        }
        (b'#', None, [_, ..]) => count_arrangements(&springs[1..], Some(1), nums, cache),
        (b'?', None, _) => {
            count_arrangements(&springs[1..], None, nums, cache)
                + count_arrangements(&springs[1..], Some(1), nums, cache)
        }
        _ => 0,
    };
    if springs[0] == b'?' {
        cache.insert((springs, in_group, nums), result);
    }
    result
}

pub(crate) fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, nums) = parse_line(l);
            count_arrangements(springs.as_bytes(), None, &nums, &mut HashMap::new())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2_first_line() {
        let example_input = "???.### 1,1,3";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn part2_second_line() {
        let example_input = ".??..??...?##. 1,1,3";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 16384);
    }

    #[test]
    fn part2() {
        let example_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let answer = crate::part2::solve(example_input);
        assert_eq!(answer, 525152);
    }
}
