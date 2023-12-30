use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/21.txt").unwrap();
    let grid = Grid::parse(&input);
    println!("Part 1: {}", solve_part_1(&grid));
}

fn solve_part_1(grid: &Grid) -> usize {
    grid.step(64).len()
}

type Position = (usize, usize);

struct Grid {
    tiles: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        Self { tiles }
    }

    fn find_start(&self) -> Position {
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                if self.tiles[row][col] == 'S' {
                    return (row, col)
                }
            }
        }
        panic!("no start");
    }

    fn step(&self, n: u32) -> HashSet<Position> {
        let mut current_positions = HashSet::from([ self.find_start() ]);
        for _ in 0..n {
            let mut next_positions = HashSet::new();
            for p in current_positions {
                if p.0 > 0 && self.can_step((p.0 - 1, p.1)) {
                    next_positions.insert((p.0 - 1, p.1));
                }
                if p.0 + 1 < self.tiles.len() && self.can_step((p.0 + 1, p.1)) {
                    next_positions.insert((p.0 + 1, p.1));
                }
                if p.1 > 0 && self.can_step((p.0, p.1 - 1)) {
                    next_positions.insert((p.0, p.1 - 1));
                }
                if p.1 + 1 < self.tiles[p.0].len() && self.can_step((p.0, p.1 + 1)) {
                    next_positions.insert((p.0, p.1 + 1));
                }
            }
            current_positions = next_positions;
        }
        current_positions
    }

    fn can_step(&self, pos: Position) -> bool {
        self.tiles[pos.0][pos.1] != '#'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let grid = Grid::parse("
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        ");

        assert_eq!(grid.find_start(), (5, 5));
        assert_eq!(grid.step(1), HashSet::from([(4, 5), (5, 4)]));
        assert_eq!(grid.step(2), HashSet::from([(3, 5), (5, 3), (5, 5), (6, 4)]));
        assert_eq!(grid.step(3), HashSet::from([(3, 6), (4, 3), (4, 5), (5, 4), (6, 3), (7, 4)]));
        assert_eq!(grid.step(6).len(), 16);
    }
}
