use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use itertools::Itertools;
use multimap::MultiMap;

fn main() {
    let input = read_to_string("data/23.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> i32 {
    let graph = DirectedGraph::parse(input);
    graph.find_longest_path()
}

fn solve_part_2(input: &str) -> i32 {
    let graph = UndirectedGraph::parse(input);
    graph.find_longest_path()
}

type Point = (usize, usize);

struct Grid {
    tiles: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Self { tiles }
    }

    fn num_rows(&self) -> usize {
        self.tiles.len()
    }

    fn num_cols(&self) -> usize {
        self.tiles[0].len()
    }

    fn get(&self, p: Point) -> char {
        self.tiles[p.0][p.1]
    }

    fn find_start(&self) -> Point {
        let row = 0;
        for col in 0..self.tiles[row].len() {
            if self.tiles[row][col] == '.' {
                return (row, col)
            }
        }
        panic!("no start");
    }

    fn find_end(&self) -> Point {
        let row = self.tiles.len() - 1;
        for col in (0..self.tiles[row].len()).rev() {
            if self.tiles[row][col] == '.' {
                return (row, col)
            }
        }
        panic!("no end");
    }
}

struct DirectedGraph {
    edges: MultiMap<Point, Point>,
    start: Point,
    end: Point,
}

impl DirectedGraph {
    fn parse(input: &str) -> Self {
        let grid = Grid::parse(input);
        let mut graph = Self { edges: MultiMap::new(), start: grid.find_start(), end: grid.find_end() };
        let mut stack = vec![graph.start];
        while let Some(current) = stack.pop() {
            let mut neighbors = Vec::new();
            let tile = grid.get(current);
            if current.0 > 0 && ['.', '^'].contains(&tile) && !['#', 'v'].contains(&grid.get((current.0 - 1, current.1))) {
                neighbors.push((current.0 - 1, current.1));
            }
            if current.0 + 1 < grid.num_rows() && ['.', 'v'].contains(&tile) && !['#', '^'].contains(&grid.get((current.0 + 1, current.1))) {
                neighbors.push((current.0 + 1, current.1));
            }
            if current.1 > 0 && ['.', '<'].contains(&tile) && !['#', '>'].contains(&grid.get((current.0, current.1 - 1))) {
                neighbors.push((current.0, current.1 - 1));
            }
            if current.1 + 1 < grid.num_cols() && ['.', '>'].contains(&tile) && !['#', '<'].contains(&grid.get((current.0, current.1 + 1))) {
                neighbors.push((current.0, current.1 + 1));
            }
            neighbors.retain(|neighbor| !graph.has_edge(&current, neighbor));
            for neighbor in neighbors {
                graph.add_edge(current, neighbor);
                stack.push(neighbor);
            }
        }
        graph
    }

    fn get_nodes(&self) -> Vec<Point> {
        self.edges.keys().cloned().collect_vec()
    }

    fn get_nodes_from(&self, from: &Point) -> Vec<Point> {
        self.edges.get_vec(from).cloned().unwrap_or_default()
    }

    fn add_edge(&mut self, from: Point, to: Point) {
        self.edges.insert(from, to);
    }

    fn has_edge(&self, from: &Point, to: &Point) -> bool {
        self.edges.get_vec(from).is_some_and(|nodes| nodes.contains(to))
    }

    fn find_longest_path(&self) -> i32 {
        let mut distances = HashMap::new();
        for node in self.get_nodes() {
            distances.insert(node, -1);
        }
        distances.insert(self.start, 0);
        let seen = HashSet::new();
        let mut queue = VecDeque::from([ (self.start, 0, seen) ]);
        while let Some((current, distance, seen)) = queue.pop_back() {
            if distance > distances[&current] {
                distances.insert(current, distance);
            }
            for next in self.get_nodes_from(&current) {
                if seen.contains(&next) {
                    continue;
                }
                let mut seen_next = seen.clone();
                seen_next.insert(current);
                queue.push_back((next, distance + 1, seen_next));
            }
        }
        distances.get(&self.end).cloned().unwrap()
    }
}

struct UndirectedGraph {
    nodes: HashSet<Point>,
    edges: HashMap<(Point, Point), i32>,
    start: Point,
    end: Point,
}

impl UndirectedGraph {
    fn parse(input: &str) -> Self {
        let grid = Grid::parse(input);
        let mut graph = Self { nodes: HashSet::new(), edges: HashMap::new(), start: grid.find_start(), end: grid.find_end() };
        let mut stack = vec![vec![graph.start]];
        let mut seen_ever = HashSet::new();
        while let Some(path) = stack.pop() {
            let mut neighbors = Vec::new();
            let head = path[0];
            let tail = path[path.len() - 1];
            if seen_ever.contains(&tail) {
                continue;
            }
            seen_ever.insert(tail);
            if tail.0 > 0 && grid.get((tail.0 - 1, tail.1)) != '#' {
                neighbors.push((tail.0 - 1, tail.1));
            }
            if tail.0 + 1 < grid.num_rows() && grid.get((tail.0 + 1, tail.1)) != '#' {
                neighbors.push((tail.0 + 1, tail.1));
            }
            if tail.1 > 0 && grid.get((tail.0, tail.1 - 1)) != '#' {
                neighbors.push((tail.0, tail.1 - 1));
            }
            if tail.1 + 1 < grid.num_cols() && grid.get((tail.0, tail.1 + 1)) != '#' {
                neighbors.push((tail.0, tail.1 + 1));
            }
            for neighbor in &neighbors {
                if graph.nodes.contains(neighbor) && !path.contains(neighbor) {
                    graph.add_edge(head, *neighbor, path.len() as i32);
                }
            }
            neighbors.retain(|neighbor| !seen_ever.contains(neighbor));
            if tail == graph.end {
                graph.add_edge(head, tail, path.len() as i32 - 1);
            } else if neighbors.len() == 1 {
                let mut path = path.clone();
                path.push(neighbors[0]);
                stack.push(path);
            } else if neighbors.len() > 1 {
                graph.add_edge(head, tail, path.len() as i32 - 1);
                for neighbor in neighbors {
                    stack.push(Vec::from([ tail, neighbor ]));
                }
            }
        }
        graph
    }

    fn get_edges(&self, from: &Point) -> Vec<(Point, i32)> {
        self.edges.iter()
            .filter(|((edge_from, _), _)| edge_from == from)
            .map(|(&(_, to), &weight)| (to, weight))
            .collect()
    }

    fn add_edge(&mut self, from: Point, to: Point, weight: i32) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        let current = self.edges.get(&(from, to)).cloned().unwrap_or_default();
        if weight > current {
            self.edges.insert((from, to), weight);
            self.edges.insert((to, from), weight);
        }
    }

    fn find_longest_path(&self) -> i32 {
        let mut distances = HashMap::new();
        for node in self.nodes.clone() {
            distances.insert(node, -1);
        }
        distances.insert(self.start, 0);
        let seen = HashSet::new();
        let mut queue = Vec::new();
        queue.push((self.start, 0, seen, Vec::new()));
        while let Some((current, distance, seen, path)) = queue.pop() {
            if distance > distances[&current] {
                distances.insert(current, distance);
            }
            for next in self.get_edges(&current) {
                if seen.contains(&next.0) {
                    continue;
                }
                let mut seen_next = seen.clone();
                seen_next.insert(current);
                let mut path = path.clone();
                path.push(current);
                queue.push((next.0, distance + next.1, seen_next, path));
            }
        }
        distances.get(&self.end).cloned().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_directed() {
        let graph = DirectedGraph::parse("
            #.####
            #.<.>.
            #v#^#.
            #...#.
        ");

        assert_eq!(graph.start, (0, 1), "start");
        assert_eq!(graph.end, (3, 5), "end");

        assert_eq!(graph.get_nodes().len(), 13, "number of nodes");

        assert!(graph.has_edge(&(0, 1), &(1, 1)), "bidirectional outgoing");
        assert!(graph.has_edge(&(1, 1), &(0, 1)), "bidirectional incoming");
        assert!(graph.has_edge(&(2, 1), &(3, 1)), "unidirectional outgoing");
        assert!(!graph.has_edge(&(3, 1), &(2, 1)), "unidirectional incoming");
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        "), 94);
    }

    #[test]
    fn test_parse_undirected() {
        let graph = UndirectedGraph::parse("
            #.####
            #.<.>.
            #v#^#.
            #...#.
        ");

        assert_eq!(graph.start, (0, 1), "start");
        assert_eq!(graph.end, (3, 5), "end");

        assert_eq!(graph.nodes.len(), 4, "number of nodes");
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2("
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        "), 154);
    }
}
