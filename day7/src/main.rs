use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    fn counts(&self) -> HashMap<Card, usize> {
        self.0.into_iter().counts()
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|n| Card::Number(n as u8))
                    .unwrap_or_else(|| match c {
                        'T' => Card::Ten,
                        'Q' => Card::Queen,
                        'K' => Card::King,
                        'A' => Card::Ace,
                        'J' => Card::Joker,
                        _ => unreachable!(),
                    })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Hand(hand))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut s = self.counts();
        let mut o = other.counts();

        let s_jokers = s
            .remove_entry(&Card::Joker)
            .map(|(_, n)| n)
            .unwrap_or_default();
        let o_jokers = o
            .remove_entry(&Card::Joker)
            .map(|(_, n)| n)
            .unwrap_or_default();

        let mut s = s.into_values().collect::<Vec<_>>();
        s.sort_by(|a, b| b.cmp(a));
        if s.is_empty() {
            s.push(0);
        }
        if let Some(n) = s.first_mut() {
            *n += s_jokers;
        }

        let mut o = o.into_values().collect::<Vec<_>>();
        o.sort_by(|a, b| b.cmp(a));
        if o.is_empty() {
            o.push(0);
        }
        if let Some(n) = o.first_mut() {
            *n += o_jokers;
        }

        match s.cmp(&o) {
            ord @ (Ordering::Greater | Ordering::Less) => ord,
            Ordering::Equal => self.0.cmp(&other.0),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut input = input
        .lines()
        .map(|l| {
            let [hand, bid] = l.split_whitespace().collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            let hand = hand.parse::<Hand>().unwrap();
            let bid = bid.parse::<u32>().unwrap();

            (hand, bid)
        })
        .collect::<Vec<_>>();

    input.sort_by(|(a, _), (b, _)| a.cmp(b));

    let res: u32 = input
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1, bid))
        .map(|(rank, bid)| rank as u32 * bid)
        .sum();

    println!("{res}");
}
