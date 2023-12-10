use std::ops::Range;

use regex::Regex;

struct Number {
    value: u32,
    row: usize,
    span: Range<usize>,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let re = Regex::new(r"\d+").unwrap();

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            re.find_iter(line).map(move |m| {
                let value = m.as_str().parse().unwrap();
                let span = m.range();

                Number { value, row, span }
            })
        })
        .collect::<Vec<_>>();

    let re = Regex::new(r"\*").unwrap();

    let gears = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| re.find_iter(line).map(move |m| (row, m.start())));

    let res: u32 = gears
        .filter_map(|coord| two_numbers(coord, &numbers))
        .map(|(a, b)| a * b)
        .sum();

    println!("{res}");
}

fn two_numbers((row, col): (usize, usize), numbers: &[Number]) -> Option<(u32, u32)> {
    let matches = &numbers
        .iter()
        .filter(|n| {
            row.abs_diff(n.row) <= 1
                && (n.span.contains(&col) || n.span.start == col + 1 || n.span.end == col)
        })
        .collect::<Vec<_>>()[..];

    if let [a, b] = matches {
        Some((a.value, b.value))
    } else {
        None
    }
}
