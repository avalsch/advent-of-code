#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let res: u32 = input
        .lines()
        .map(parse_game_maxs)
        .map(|Game { red, green, blue }| red * green * blue)
        .sum();

    println!("{res}");
}

fn parse_game_maxs(game: &str) -> Game {
    let [_, records] = game.split(':').collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    records
        .split([',', ';'])
        .fold(Game::default(), |game, record| {
            let [count, color] = record.split_whitespace().collect::<Vec<_>>()[..] else {
                unreachable!()
            };

            let count = count.parse().unwrap();

            match color {
                "red" => Game {
                    red: game.red.max(count),
                    ..game
                },
                "green" => Game {
                    green: game.green.max(count),
                    ..game
                },
                "blue" => Game {
                    blue: game.blue.max(count),
                    ..game
                },
                _ => unreachable!(),
            }
        })
}
