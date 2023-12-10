fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let [time, distance] = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<u64>().unwrap())
                .fold(0, |aggr, n| format!("{aggr}{n}").parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>()[..]
    else {
        unreachable!()
    };

    let res: usize = (0..time)
        .map(move |t| t * time.abs_diff(t))
        .filter(move |d| *d > distance)
        .count();

    println!("{res}");
}
