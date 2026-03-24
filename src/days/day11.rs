use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

pub fn run(input: &str) -> usize {
    let empty_graph: HashMap<&str, Vec<&str>> = HashMap::new();

    let graph = input
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
        });

    let mut paths = HashMap::new();
    let from = "svr";
    let to = "out";
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
// if vertex == "dac" {
//     return 0;
// let dac = visited.contains("dac");
// let fft = visited.contains("fft");
// if dac && fft {
//     return 1;
// } else {
//     return 0;
// }
// }
