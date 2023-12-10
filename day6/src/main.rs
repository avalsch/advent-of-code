fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let [times, distances] = &input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()[..]
    else {
        unreachable!()
    };

    let records = times.iter().zip(distances);

    let res: usize = records
        .map(|(&time, &distance)| {
            (0..time)
                .map(move |t| t * time.abs_diff(t))
                .filter(move |d| *d > distance)
                .count()
        })
        .product();

    println!("{res}");
}
