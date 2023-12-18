use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

fn main() {
    let input = read_to_string("data/14.txt").unwrap();
    let mut platform = Platform::parse(&input);
    println!("Part 1: {}", solve_part_1(&mut platform));
    println!("Part 2: {}", solve_part_2(&mut platform));
}

fn solve_part_1(platform: &mut Platform) -> u32 {
    platform.tilt();
    platform.calculate_load()
}

fn solve_part_2(platform: &mut Platform) -> u32 {
    let num_cycles = 1_000_000_000;
    let mut seen = HashMap::new();
    for i in 0..num_cycles {
        platform.cycle();
        let hash = platform.get_hash();
        match seen.get(&hash) {
            Some(i0) => {
                for _ in 0..((num_cycles - i) % (i - i0) - 1) {
                    platform.cycle();
                }
                break;
            },
            None => {
                seen.insert(hash, i);
            },
        }
    }
    platform.calculate_load()
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Platform {
    grid: Vec<Vec<char>>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let grid = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        Self { grid }
    }

    fn calculate_load(&self) -> u32 {
        let mut load = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == 'O' {
                    load += self.grid.len() - row;
                }
            }
        }
        load as u32
    }

    fn tilt(&mut self) {
        for row in 0..self.grid.len() {
            'col: for col in 0..self.grid[row].len() {
                if self.grid[row][col] == '.' {
                    for swap_row in (row + 1)..self.grid.len() {
                        match self.grid[swap_row][col] {
                            '#' => continue 'col,
                            'O' => {
                                (self.grid[row][col], self.grid[swap_row][col]) = (self.grid[swap_row][col], self.grid[row][col]);
                                continue 'col;
                            },
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    fn rotate(&mut self) {
        let n = self.grid.len();
        for layer in 0..n / 2 {
            let first = layer;
            let last = n - 1 - layer;
            for i in first..last {
                let offset = i - first;
                let top = self.grid[first][i];
                self.grid[first][i] = self.grid[last - offset][first]; // left to top
                self.grid[last - offset][first] = self.grid[last][last - offset]; // bottom to left
                self.grid[last][last - offset] = self.grid[i][last]; // right to bottom
                self.grid[i][last] = top; // top to right
            }
        }
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate();
        }
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.grid.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Platform {
        Platform::parse("
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        ")
    }

    #[test]
    fn test_calculate_load() {
        let platform = Platform::parse("
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        ");
        assert_eq!(platform.calculate_load(), 136);
    }

    #[test]
    fn test_tilt() {
        let mut platform = setup();
        platform.tilt();
        assert_eq!(platform, Platform::parse("
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        "));
    }

    #[test]
    fn test_solve_part_1() {
        let mut platform = setup();
        assert_eq!(solve_part_1(&mut platform), 136);
    }

    #[test]
    fn test_rotate() {
        let mut platform = Platform::parse("
            0..
            .0.
            ..#
        ");
        platform.rotate();
        assert_eq!(platform, Platform::parse("
            ..0
            .0.
            #..
        "));
    }

    #[test]
    fn test_cycle_one_time() {
        let mut platform = setup();
        platform.cycle();
        assert_eq!(platform, Platform::parse("
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "));
    }

    #[test]
    fn test_cycle_two_times() {
        let mut platform = setup();
        platform.cycle();
        platform.cycle();
        assert_eq!(platform, Platform::parse("
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        "));
    }

    #[test]
    fn test_cycle_three_times() {
        let mut platform = setup();
        platform.cycle();
        platform.cycle();
        platform.cycle();
        assert_eq!(platform, Platform::parse("
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        "));
    }

    #[test]
    fn test_solve_part_2() {
        let mut platform = setup();
        assert_eq!(solve_part_2(&mut platform), 64);
    }
}
