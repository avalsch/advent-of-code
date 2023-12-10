use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut lines = input.lines().peekable();

    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = HashMap::new();

    lines.next().unwrap();

    while lines.peek().is_some() {
        let header = lines.next().unwrap().split_whitespace().next().unwrap();
        let mut ranges = vec![];

        while lines.peek().is_some_and(|s| !s.is_empty()) {
            let line = lines.next().unwrap();

            let [dest, source, range] = line
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                unreachable!()
            };

            let source_range = source..source + range;
            let offset = dest as i64 - source as i64;

            ranges.push((source_range, offset))
        }

        maps.insert(header, ranges);

        lines.next();
    }

    let locations = seeds
        .iter()
        .copied()
        .map(|seed| get_dest(seed, &maps["seed-to-soil"]))
        .map(|soil| get_dest(soil, &maps["soil-to-fertilizer"]))
        .map(|fert| get_dest(fert, &maps["fertilizer-to-water"]))
        .map(|water| get_dest(water, &maps["water-to-light"]))
        .map(|light| get_dest(light, &maps["light-to-temperature"]))
        .map(|temp| get_dest(temp, &maps["temperature-to-humidity"]))
        .map(|humidity| get_dest(humidity, &maps["humidity-to-location"]));

    let res = locations.min().unwrap();

    println!("{res}");
}

fn get_dest(source: u64, offsets: &[(Range<u64>, i64)]) -> u64 {
    offsets
        .iter()
        .find(|(range, _)| range.contains(&source))
        .map(|(_, offset)| (source as i64 + offset) as u64)
        .unwrap_or(source)
}
