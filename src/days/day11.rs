use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    traverse_all_paths("you", "out", HashSet::new(), &create_graph(input))
}
pub fn part2(input: &str) -> usize {
    traverse_all_paths(
        "svr",
        "out",
        HashSet::from(["dac", "fft"]),
        &create_graph(input),
    )
}

fn create_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let empty_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    input
        .lines()
        .chain(once("out: "))
        .fold(empty_graph, |mut acc_state, line| {
            let (label, nodes) = match line.split_once(":") {
                Some((label, nodes)) => (
                    label,
                    nodes
                        .trim()
                        .split(" ")
                        .filter(|s| s.len() > 0)
                        .collect(),
                ),
                None => panic!("Malformed input"),
            };
            acc_state.insert(label, nodes);
            acc_state
        })
}

fn traverse_all_paths(
    from: &str,
    to: &str,
    via: HashSet<&str>,
    graph: &HashMap<&str, Vec<&str>>,
) -> usize {
    let paths = via
        .iter()
        .combinations(via.len());
    paths.fold(0, |sum, path| {
        sum + once(from)
            .chain(
                path.iter()
                    .map(|v| **v),
            )
            .chain(once(to))
            .tuple_windows()
            .fold(1, |acc, (f, t)| acc * from_to(f, t, graph))
    })
}

fn from_to(from: &str, to: &str, graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut paths = HashMap::new();
    paths.insert(to, 1);

    dfs_sum(&graph, &mut paths, from, &HashSet::new());
    let result = paths
        .get(from)
        .expect(format!("{from} should exist in paths").as_str());
    *result
}

fn dfs_sum<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    paths: &mut HashMap<&'a str, usize>,
    vertex: &'a str,
    visited: &HashSet<&'a str>,
) {
    let Some(nodes) = graph.get(vertex) else {
        panic!("No node with name: {vertex}")
    };

    match paths
        .get(vertex)
        .cloned()
    {
        Some(visits) => {
            for vis in visited {
                paths
                    .entry(vis)
                    .and_modify(|counter| *counter += visits);
            }
        }
        None => {
            let mut new_visited = visited.clone();
            new_visited.insert(vertex);
            paths.insert(vertex, 0);

            for node in nodes {
                dfs_sum(graph, paths, node, &new_visited)
            }
        }
    }
}

#[allow(dead_code)]
fn dfs(
    graph: &HashMap<&str, Vec<&str>>,
    vertex: &str,
    target: &str,
    visited: &HashSet<&str>,
) -> bool {
    if vertex == target {
        //println!("{:#?}", visited);
        return true;
    }

    let Some(nodes) = graph.get(vertex) else {
        panic!("No node with name: {vertex}")
    };

    let mut new_visited = visited.clone();

    for node in nodes.into_iter() {
        if !new_visited.contains(node) {
            new_visited.insert(node);
            println!("{:#?}", new_visited);
            if dfs(graph, node, target, &new_visited) {
                return true;
            }
        }
    }
    println!("No hits!");
    false
}
