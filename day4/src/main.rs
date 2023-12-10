fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let res: u32 = input.lines().map(card_points).sum();

    println!("{res}");
}

fn card_points(card: &str) -> u32 {
    let [_card, numbers, winners] = &card.split([':', '|']).collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let winners = winners
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let shift = numbers
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .filter(|n| winners.contains(n))
        .count();

    if shift > 0 {
        1 << (shift - 1)
    } else {
        0
    }
}
