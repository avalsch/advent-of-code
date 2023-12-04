fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let res: u32 = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|v| match v.as_slice() {
            [f] => 11 * f,
            [f, .., l] => 10 * f + l,
            _ => unreachable!(),
        })
        .sum();

    println!("{res}");
}
