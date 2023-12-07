// Advent of Code - Day 7: Camel Cards Part 2

use crate::part1::Type;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        let joker_counts = value.iter().filter(|c| *c == &Card::Joker).count() as u32;
        // count cards except Jokers
        let counts: HashMap<&Card, u32> =
            value
                .iter()
                .filter(|c| *c != &Card::Joker)
                .fold(HashMap::new(), |mut counts, card| {
                    *counts.entry(card).or_insert(0) += 1;
                    counts
                });
        let mut counts = counts.values().cloned().collect::<Vec<u32>>();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        // add joker count to highest count
        if let Some(first) = counts.first_mut() {
            *first += joker_counts;
        }
        match counts.as_slice() {
            [] => Type::FiveOfAKind, // Hand only contained Jokers
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, ..] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, ..] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            count => panic!("Invalid count: `{count:?}`"),
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

pub(crate) fn solve_part_two(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::from).collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.winnings(rank as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let example_input = include_str!("./example.txt");
        let answer = crate::part2::solve_part_two(example_input);
        assert_eq!(answer, 5905);
    }
}
