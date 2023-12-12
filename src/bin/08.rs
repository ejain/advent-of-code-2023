use std::collections::HashMap;
use std::fs::read_to_string;
use num::integer::lcm;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/08.txt").unwrap();
    let (instructions, nodes) = parse(&input);
    println!("Part 1: {}", solve_part_1(&instructions, &nodes));
    println!("Part 2: {}", solve_part_2(&instructions, &nodes));
}

fn parse(input: &str) -> (String, HashMap<String, Node>) {
    let mut instructions = String::new();
    let mut nodes = HashMap::new();
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(cap) = regex!(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").captures(line) {
            let (_, [label, left, right]) = cap.extract();
            nodes.insert(label.to_string(), Node::new(label, left, right));
        } else if regex!(r"([LR]+)").is_match(line) {
            instructions = line.to_string();
        } else {
            panic!("don't know how to parse <{}>", line);
        }
    }
    (instructions, nodes)
}

fn solve_part_1(instructions: &str, nodes: &HashMap<String, Node>) -> u64 {
    count_steps(&nodes["AAA"], |node| node.label == "ZZZ", instructions, nodes)
}

fn solve_part_2(instructions: &str, nodes: &HashMap<String, Node>) -> u64 {
    nodes.values()
        .filter(|node| node.is_start())
        .map(|start_node| count_steps(start_node, |node| node.is_end(), instructions, nodes))
        .reduce(lcm)
        .unwrap()
}

fn count_steps(start_node: &Node, is_end: fn(&Node) -> bool, instructions: &str, nodes: &HashMap<String, Node>) -> u64 {
    let mut steps = 0;
    let mut i = instructions.chars().cycle();
    let mut current_node = start_node;
    while !is_end(current_node) {
        match i.next().unwrap() {
            'L' => current_node = &nodes[&current_node.left],
            'R' => current_node = &nodes[&current_node.right],
            c => panic!("invalid instruction <{}>", c),
        }
        steps += 1;
    }
    steps
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String
}

impl Node {
    fn new(label: &str, left: &str, right: &str) -> Node {
        Node { label: label.to_string(), left: left.to_string(), right: right.to_string() }
    }

    fn is_start(&self) -> bool {
        self.label.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.label.ends_with('Z')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let (instructions, nodes) = parse("
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ");
        assert_eq!(solve_part_1(&instructions, &nodes), 2);
    }

    #[test]
    fn test_solve_part_1_with_repeats() {
        let (instructions, nodes) = parse("
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ");
        assert_eq!(solve_part_1(&instructions, &nodes), 6);
    }

    #[test]
    fn test_solve_part_2() {
        let (instructions, nodes) = parse("
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ");
        assert_eq!(solve_part_2(&instructions, &nodes), 6);
    }
}
