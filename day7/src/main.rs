use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Eq)]
struct Hand([u8; 5]);

impl Hand {
    fn counts(&self) -> HashMap<u8, usize> {
        self.0.into_iter().counts()
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s
            .chars()
            .map(|c| {
                c.to_digit(10).map(|n| n as u8).unwrap_or_else(|| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => unreachable!(),
                })
            })
            .collect::<Vec<_>>();
        let hand = hand.try_into().unwrap();

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
        let mut s = self.counts().into_values().collect::<Vec<_>>();
        s.sort_by(|a, b| b.cmp(a));

        let mut o = other.counts().into_values().collect::<Vec<_>>();
        o.sort_by(|a, b| b.cmp(a));

        let ord = s.cmp(&o);

        match ord {
            Ordering::Greater | Ordering::Less => ord,
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
