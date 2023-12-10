fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut copies = (0..input.lines().count()).map(|_| 1).collect::<Vec<_>>();

    for (i, card) in input.lines().enumerate() {
        let card_copies = copies[i];

        let [_card, numbers, winners] = &card.split([':', '|']).collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        let winners = winners
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        let winners = numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winners.contains(n))
            .count();

        copies[(i+1)..=(i + winners)]
            .iter_mut()
            .for_each(|c| *c += card_copies);
    }

    let res: usize = copies.iter().sum();

    println!("{res}");
}
