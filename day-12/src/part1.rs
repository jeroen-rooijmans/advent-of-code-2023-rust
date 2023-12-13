// Advent of Code - Day 12: Hot Springs Part 1

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if let (Some(springs), Some(nums)) = (parts.first(), parts.last()) {
        let nums = nums.split(',').filter_map(|n| n.parse().ok()).collect();
        (String::from(*springs), nums)
    } else {
        panic!("Invalid input!");
    }
}

pub(crate) fn count_groups(input: &str) -> Vec<usize> {
    input
        .split('.')
        .filter(|&group| group.contains('#'))
        .map(|group| group.len())
        .collect()
}

fn generate_arrangements(input: &str) -> Vec<String> {
    input.chars().fold(vec!["".to_string()], |acc, c| {
        acc.into_iter()
            .flat_map(|s| match c {
                '?' => vec![format!("{}.", s), format!("{}#", s)],
                _ => vec![format!("{}{}", s, c)],
            })
            .collect()
    })
}

pub(crate) fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, nums) = parse_line(l);
            generate_arrangements(&springs)
                .iter()
                .filter(|&s| count_groups(s) == nums)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 21);
    }
}
