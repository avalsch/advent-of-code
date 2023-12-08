fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let res: u32 = input
        .lines()
        .map(parse_digits)
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum();

    println!("{res}");
}

fn parse_digits(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];

    for sub in (0..line.len()).map(|i| &line[i..]) {
        let digit = sub.chars().next().unwrap().to_digit(10).or_else(|| {
            let digit = match sub {
                _ if sub.starts_with("one") => 1,
                _ if sub.starts_with("two") => 2,
                _ if sub.starts_with("three") => 3,
                _ if sub.starts_with("four") => 4,
                _ if sub.starts_with("five") => 5,
                _ if sub.starts_with("six") => 6,
                _ if sub.starts_with("seven") => 7,
                _ if sub.starts_with("eight") => 8,
                _ if sub.starts_with("nine") => 9,
                _ => return None,
            };

            Some(digit)
        });

        if let Some(digit) = digit {
            digits.push(digit);
        }
    }

    digits
}
