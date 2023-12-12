use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let _ = lines.next();

    let mut directions = directions
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        })
        .cycle();

    let map = lines
        .map(|line| {
            let [node, connections] = line.split('=').collect::<Vec<_>>()[..] else {
                unreachable!()
            };

            let node = node.trim().to_string();
            let [l, r] = connections[2..10].trim().split(',').collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            let l = l.trim().to_string();
            let r = r.trim().to_string();

            (node, (l, r))
        })
        .collect::<HashMap<String, (String, String)>>();

    let mut node = &map["AAA"];
    let mut steps = 0;

    loop {
        steps += 1;
        let next = match directions.next().unwrap() {
            Dir::Left => &node.0,
            Dir::Right => &node.1,
        };
        if next != "ZZZ" {
            node = &map[next];
        } else {
            break;
        }
    }

    println!("{steps}");
}
