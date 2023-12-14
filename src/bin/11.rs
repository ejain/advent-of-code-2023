use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Range;
use itertools::Itertools;

fn main() {
    let input = read_to_string("data/11.txt").unwrap();
    let universe = Universe::parse(&input);
    println!("Part 1: {}", solve(&universe, 2));
    println!("Part 2: {}", solve(&universe, 1_000_000));
}

fn solve(universe: &Universe, expansion_factor: u64) -> u64 {
    universe.find_galaxies().iter()
        .combinations(2)
        .map(|pair| universe.distance(pair[0], pair[1], expansion_factor))
        .sum()
}

#[derive(Debug, PartialEq)]
struct Universe {
    space: Vec<Vec<char>>,
}

impl Universe {
    const EMPTY: char = '.';
    const GALAXY: char = '#';

    fn parse(input: &str) -> Universe {
        let mut space = Vec::new();
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
            space.push(line.chars().collect());
        }
        Universe { space }
    }

    fn find_galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for row in 0..self.space.len() {
            for col in 0..self.space[row].len() {
                if self.space[row][col] == Universe::GALAXY {
                    galaxies.push((row, col));
                }
            }
        }
        galaxies
    }

    fn distance(&self, from: &(usize, usize), to: &(usize, usize), expansion_factor: u64) -> u64 {
        let mut distance = ((to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()) as u64;
        distance += self.expansion_between(from.0, to.0, &self.empty_rows(), expansion_factor);
        distance += self.expansion_between(from.1, to.1, &self.empty_cols(), expansion_factor);
        distance
    }

    fn empty_rows(&self) -> HashSet<usize> {
        let mut empty_rows = HashSet::new();
        for row in 0..self.space.len() {
            if self.space[row].iter().all(|object| object == &Universe::EMPTY) {
                empty_rows.insert(row);
            }
        }
        empty_rows
    }

    fn empty_cols(&self) -> HashSet<usize> {
        let mut empty_cols = HashSet::new();
        for col in 0..self.space[0].len() {
            if (0..self.space.len()).all(|row| self.space[row][col] == Universe::EMPTY) {
                empty_cols.insert(col);
            }
        }
        empty_cols
    }

    fn expansion_between(&self, from: usize, to: usize, expandable_positions: &HashSet<usize>, expansion_factor: u64) -> u64 {
        let intersects = normalized_range(from, to).filter(|row| expandable_positions.contains(row)).count() as u64;
        intersects * (expansion_factor - 1)
    }
}

fn normalized_range(from: usize, to: usize) -> Range<usize> {
    if from < to {
        from..to
    } else {
        to..from
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    fn setup() -> Universe {
        Universe::parse("
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ")
    }

    #[test]
    fn test_find_galaxies() {
        let universe = setup();
        assert_eq!(universe.find_galaxies(), vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0), (9, 4)
        ]);
    }

    #[test]
    fn test_find_empty() {
        let universe = setup();
        assert_eq!(universe.empty_rows(), HashSet::from([3, 7]));
        assert_eq!(universe.empty_cols(), HashSet::from([2, 5, 8]));
    }

    #[test_case((0, 0), (0, 0), 1, 0; "same")]
    #[test_case((0, 0), (0, 4), 1, 4; "right")]
    #[test_case((0, 0), (0, 4), 3, 6; "right and expand")]
    #[test_case((0, 0), (4, 0), 1, 4; "down")]
    #[test_case((0, 0), (4, 0), 3, 6; "down and expand")]
    #[test_case((1, 1), (3, 4), 1, 5; "right and down")]
    #[test_case((3, 4), (1, 1), 1, 5; "up and left")]
    fn test_distance(from: (usize, usize), to: (usize, usize), expansion_factor: u64, expected: u64) {
        let universe = setup();
        assert_eq!(universe.distance(&from, &to, expansion_factor), expected);
    }

    #[test_case(2, 374; "x2")]
    #[test_case(10, 1030; "x10")]
    #[test_case(100, 8410; "x100")]
    fn test_solve(expansion_factor: u64, expected: u64) {
        let universe = setup();
        assert_eq!(solve(&universe, expansion_factor), expected);
    }
}
