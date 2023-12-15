// Advent of Code - Day 15: Lens Library Part 1

/// Apply the HASH algorithm on a label
pub(crate) fn hash(label: &str) -> usize {
    label
        .bytes()
        .fold(0, |acc, ascii_code| (acc + ascii_code as usize) * 17 % 256)
}

pub(crate) fn solve(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_hash() {
        let example_input = "HASH";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 52);
    }

    #[test]
    fn part1() {
        let example_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 1320);
    }

    #[test]
    fn part1_step1() {
        let example_input = "rn=1";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 30);
    }

    #[test]
    fn part1_step2() {
        let example_input = "cm-";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 253);
    }

    #[test]
    fn part1_step3() {
        let example_input = "qp=3";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 97);
    }

    #[test]
    fn part1_step4() {
        let example_input = "cm=2";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 47);
    }

    #[test]
    fn part1_step5() {
        let example_input = "qp-";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 14);
    }

    #[test]
    fn part1_step6() {
        let example_input = "pc=4";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 180);
    }

    #[test]
    fn part1_step7() {
        let example_input = "ot=9";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 9);
    }

    #[test]
    fn part1_step8() {
        let example_input = "ab=5";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 197);
    }

    #[test]
    fn part1_step9() {
        let example_input = "pc-";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 48);
    }

    #[test]
    fn part1_step10() {
        let example_input = "pc=6";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 214);
    }

    #[test]
    fn part1_step11() {
        let example_input = "ot=7";
        let answer = crate::part1::solve(example_input);
        assert_eq!(answer, 231);
    }
}
