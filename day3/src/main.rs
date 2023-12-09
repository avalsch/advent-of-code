use std::ops::Range;

use regex::Regex;

struct Number {
    value: u32,
    row: usize,
    span: Range<usize>,
}

fn add_col_left(row: usize, start: usize, indicies: &mut Vec<(usize, usize)>) {
    if let Some(col) = start.checked_sub(1) {
        indicies.push((row, col));
    }
}

fn add_col_right(row: usize, len: usize, col: usize, indicies: &mut Vec<(usize, usize)>) {
    if col < len {
        indicies.push((row, col));
    }
}

impl Number {
    fn is_valid(&self, arr: &[Vec<char>]) -> bool {
        let Range { start, end } = self.span.clone();
        let row = self.row;

        let mut indicies = vec![];

        add_col_left(row, start, &mut indicies);
        add_col_right(row, arr[row].len(), end, &mut indicies);

        // We have a row above
        if let Some(row) = row.checked_sub(1) {
            indicies.extend(self.span.clone().map(|i| (row, i)));

            add_col_left(row, start, &mut indicies);
            add_col_right(row, arr[row].len(), end, &mut indicies);
        }

        // We have a row below
        if row + 1 < arr.len() {
            let row = row + 1;
            indicies.extend(self.span.clone().map(|i| (row, i)));

            add_col_left(row, start, &mut indicies);
            add_col_right(row, arr[row].len(), end, &mut indicies);
        }

        indicies
            .into_iter()
            .map(|(row, col)| arr[row][col])
            .any(|c| !matches!(c, '0'..='9' | '.'))
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let array = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let re = Regex::new(r"\d+").unwrap();

    let numbers = input.lines().enumerate().flat_map(|(row, line)| {
        re.find_iter(line).map(move |m| {
            let value = m.as_str().parse().unwrap();
            let span = m.range();

            Number { value, row, span }
        })
    });

    let res: u32 = numbers
        .filter(|n| n.is_valid(&array))
        .map(|n| n.value)
        .sum();

    println!("{res}");
}
