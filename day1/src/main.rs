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
    let mut index = 0;

    while index < line.len() {
        let digit = line
            .chars()
            .nth(index)
            .unwrap()
            .to_digit(10)
            .unwrap_or_else(|| {
                let sub = &line[index..];
                match sub {
                    _ if sub.starts_with("one") => 1,
                    _ if sub.starts_with("two") => 2,
                    _ if sub.starts_with("three") => 3,
                    _ if sub.starts_with("four") => 4,
                    _ if sub.starts_with("five") => 5,
                    _ if sub.starts_with("six") => 6,
                    _ if sub.starts_with("seven") => 7,
                    _ if sub.starts_with("eight") => 8,
                    _ if sub.starts_with("nine") => 9,
                    _ => 0,
                }
            });

        if digit != 0 {
            digits.push(digit);
        }

        index += 1;
    }

    digits
}
