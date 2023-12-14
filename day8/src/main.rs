use std::collections::HashMap;

use num::Integer;
use petgraph::visit::EdgeRef;
use petgraph::Graph;

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

    let mut nodes = HashMap::new();
    let mut graph = Graph::new();

    for line in lines {
        let [node, connections] = line.split(" = ").collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        let node = node.to_owned();
        let [left, right] = connections[1..9].split(", ").collect::<Vec<_>>()[..] else {
            unreachable!()
        };
        let left = left.to_owned();
        let right = right.to_owned();

        let node = *nodes
            .entry(node.clone())
            .or_insert_with(|| graph.add_node(node));

        let left = *nodes
            .entry(left.clone())
            .or_insert_with(|| graph.add_node(left));
        let right = *nodes
            .entry(right.clone())
            .or_insert_with(|| graph.add_node(right));

        graph.add_edge(node, left, Dir::Left);
        graph.add_edge(node, right, Dir::Right);
    }

    let directions = directions
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        })
        .cycle();

    let nodes = graph.node_indices().filter(|&i| graph[i].ends_with('A'));

    let steps = nodes
        .map(|mut node| {
            let mut steps: u64 = 0;
            let mut dirs = directions.clone();

            while !graph[node].ends_with('Z') {
                let dir = dirs.next().unwrap();
                node = graph
                    .edges(node)
                    .find(|e| *e.weight() == dir)
                    .unwrap()
                    .target();

                steps += 1;
            }

            steps
        })
        .fold(1, |a, b| a.lcm(&b));

    println!("{steps}");
}
