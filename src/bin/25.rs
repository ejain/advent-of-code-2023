use std::fs::read_to_string;
use counter::Counter;
use itertools::Itertools;
use petgraph::{algo, Undirected};
use petgraph::algo::connected_components;
use petgraph::graphmap::GraphMap;
use petgraph::prelude::UnGraphMap;
use petgraph::visit::Dfs;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/25.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
}

fn solve_part_1(input: &str) -> u32 {
    let mut graph = parse(input);
    assert_eq!(connected_components(&graph), 1);

    for (edge, _) in most_common_edges(&graph).iter().take(3) {
        graph.remove_edge(edge.0, edge.1);
    }
    assert_eq!(connected_components(&graph), 2);

    let reachable = count_connected(&graph, graph.nodes().last().unwrap());
    reachable * (graph.nodes().count() as u32 - reachable)
}

fn parse(input: &str) -> GraphMap<&str, usize, Undirected> {
    let mut graph = UnGraphMap::new();
    let re = regex!(r"[a-z]+");
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut labels: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let from = labels.remove(0);
        for to in labels {
            graph.add_edge(from, to, 1usize);
        }
    }
    graph
}

fn most_common_edges<'a>(graph: &GraphMap<&'a str, usize, Undirected>) -> Vec<((&'a str, &'a str), usize)> {
    let mut edges: Counter<(&str, &str)> = Counter::new();
    for pair in graph.nodes().combinations(2).take(50_000) {
        if let Some((_cost, path)) = algo::astar(&graph, pair[0], |n| n == pair[1], |_| 1, |_| 0) {
            for edge in path.windows(2) {
                let edge = if edge[0] <= edge[1] { (edge[0], edge[1]) } else { (edge[1], edge[0]) };
                edges[&edge] += 1;
            }
        }
    }
    edges.most_common()
}

fn count_connected(graph: &GraphMap<&str, usize, Undirected>, node: &str) -> u32 {
    let mut dfs = Dfs::new(graph, node);
    let mut count = 0;
    while let Some(_node) = dfs.next(graph) {
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        "), 54);
    }
}
