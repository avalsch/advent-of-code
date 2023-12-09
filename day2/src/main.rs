fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let res: usize = input
        .lines()
        .enumerate()
        .filter(|(_, game)| valid_game(game))
        .map(|(i, _)| i + 1)
        .sum();

    println!("{res}");
}

fn valid_game(game: &str) -> bool {
    let [_, records] = &game.split(':').collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    records.split([',', ';']).all(|record| {
        let [count, color] = &record.split_whitespace().collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        let count = count.parse::<u32>().unwrap();

        matches!(
            (count, *color),
            (..=12, "red") | (..=13, "green") | (..=14, "blue")
        )
    })
}
