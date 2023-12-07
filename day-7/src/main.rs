// Advent of Code - Day 7: Camel Cards

use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    TreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        let counts = value.iter().fold(HashMap::new(), |mut counts, card| {
            *counts.entry(card).or_insert(0) += 1;
            counts
        });
        match (
            counts.values().any(|&c| c == 5),
            counts.values().any(|&c| c == 4),
            counts.values().any(|&c| c == 3),
            counts.values().any(|&c| c == 2),
            counts.values().filter(|&&count| count == 2).count(),
        ) {
            (true, _, _, _, _) => Type::FiveOfAKind,
            (_, true, _, _, _) => Type::FourOfAKind,
            (_, _, true, true, _) => Type::FullHouse,
            (_, _, true, _, _) => Type::TreeOfAKind,
            (_, _, _, _, 2) => Type::TwoPair,
            (_, _, _, _, 1) => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    hand_type: Type,
    bid: u32,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut itr = value.split_whitespace();
        let cards = itr
            .next()
            .unwrap()
            .chars()
            .map(Card::from)
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let hand_type = Type::from(&cards);
        let bid = itr.next().unwrap().parse::<u32>().unwrap();
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_ordering = self.hand_type.partial_cmp(&other.hand_type);
        if type_ordering.is_some() && type_ordering != Some(std::cmp::Ordering::Equal) {
            return type_ordering;
        }
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let card_ordering = self_card.partial_cmp(other_card);
            if card_ordering.is_some() && card_ordering != Some(std::cmp::Ordering::Equal) {
                return card_ordering;
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hand {
    fn winnings(&self, rank: u32) -> u32 {
        self.bid * rank
    }
}

fn solve_part_one(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::from).collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.winnings(rank as u32 + 1))
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    unimplemented!();
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
        assert_eq!(answer, 6440);
    }

    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, todo!());
    }
}
