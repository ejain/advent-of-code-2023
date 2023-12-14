use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/10.txt").unwrap();
    let grid = Grid::parse(&input);
    println!("Part 1: {}", solve_part_1(&grid));
    println!("Part 2: {}", solve_part_2(&grid));
}

fn solve_part_1(grid: &Grid) -> u32 {
    let path = grid.find_loop();
    path.len() as u32 / 2 + path.len() as u32 % 2
}

fn solve_part_2(grid: &Grid) -> u32 {
    let path = grid.find_loop();
    let mut enclosed = 0;
    for p in grid.find_all() {
        if !path.contains(&p) && grid.is_enclosed(p, &path) {
            enclosed += 1;
        }
    }
    enclosed
}

#[derive(Debug, PartialEq)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Start,
    NorthToSouth,
    EastToWest,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'S' => Tile::Start,
            '|' => Tile::NorthToSouth,
            '-' => Tile::EastToWest,
            'L' => Tile::NorthToEast,
            'J' => Tile::NorthToWest,
            '7' => Tile::SouthToWest,
            'F' => Tile::SouthToEast,
            '.' => Tile::Ground,
            _ => panic!("invalid tile <{}>", c),
        }
    }

    fn connects_to(&self, h: Heading) -> bool {
        match self {
            Tile::Start => true,
            Tile::NorthToSouth => h == Heading::North || h == Heading::South,
            Tile::EastToWest => h == Heading::East || h == Heading::West,
            Tile::NorthToEast => h == Heading::North || h == Heading::East,
            Tile::NorthToWest => h == Heading::North || h == Heading::West,
            Tile::SouthToWest => h == Heading::South || h == Heading::West,
            Tile::SouthToEast => h == Heading::South || h == Heading::East,
            Tile::Ground => false,
        }
    }
}

type Point = (usize, usize);

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut tiles = Vec::new();
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
             tiles.push(line.chars().map(Tile::from_char).collect());
        }
        Grid { tiles }
    }

    fn find_loop(&self) -> HashSet<Point> {
        let start = self.find_start();
        let mut visited = HashSet::from([start]);
        let mut current = start;
        'outer: loop {
            for adjacent in self.adjacent(current) {
                if !self.is_connected(current, adjacent) {
                    continue;
                }
                if adjacent == start && visited.len() > 3 {
                    break 'outer;
                }
                if !visited.contains(&adjacent) {
                    current = adjacent;
                    visited.insert(current);
                    continue 'outer;
                }
            }
            panic!("stuck at {:?}", current);
        }
        visited
    }

    fn find_start(&self) -> Point {
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                if self.tiles[row][col] == Tile::Start {
                    return (row, col)
                }
            }
        }
        panic!("no start");
    }

    fn get(&self, p: Point) -> Tile {
        assert!(self.contains(p));
        self.tiles[p.0][p.1]
    }

    fn contains(&self, p: Point) -> bool {
        (0..self.tiles.len()).contains(&(p.0)) && (0..self.tiles[0].len()).contains(&(p.1))
    }

    fn is_connected(&self, from: Point, to: Point) -> bool {
        let (from_heading, to_heading) = if from.0 + 1 == to.0 && from.1 == to.1 {
            (Heading::South, Heading::North)
        } else if from.0 > 0 && from.0 - 1 == to.0 && from.1 == to.1 {
            (Heading::North, Heading::South)
        } else if from.0 == to.0 && from.1 + 1 == to.1 {
            (Heading::East, Heading::West)
        } else if from.0 == to.0 && from.1 > 0 && from.1 - 1 == to.1 {
            (Heading::West, Heading::East)
        } else {
            panic!("{:?} and {:?} are not adjacent", from, to)
        };
        self.get(from).connects_to(from_heading) && self.get(to).connects_to(to_heading)
    }

    fn adjacent(&self, p: Point) -> Vec<Point> {
        let mut adjacent = Vec::new();
        if p.0 > 0 && self.contains((p.0 - 1, p.1)) {
            adjacent.push((p.0 - 1, p.1));
        }
        if self.contains((p.0 + 1, p.1)) {
            adjacent.push((p.0 + 1, p.1));
        }
        if p.1 > 0 && self.contains((p.0, p.1 - 1)) {
            adjacent.push((p.0, p.1 - 1));
        }
        if self.contains((p.0, p.1 + 1)) {
            adjacent.push((p.0, p.1 + 1));
        }
        adjacent
    }

    fn find_all(&self) -> Vec<Point> {
        let mut ground = Vec::new();
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                ground.push((row, col))
            }
        }
        ground
    }

    fn is_enclosed(&self, p: Point, path: &HashSet<Point>) -> bool {
        let mut intersects = 0;
        let mut last_turn = None;
        for col in p.0..self.tiles.len() {
            let next = (col, p.1);
            if !path.contains(&next) {
                continue;
            }
            match self.get(next) {
                Tile::EastToWest => {
                    intersects += 1;
                    last_turn = None;
                },
                Tile::SouthToEast => last_turn = Some(Tile::SouthToEast),
                Tile::SouthToWest => last_turn = Some(Tile::SouthToWest),
                Tile::NorthToWest => {
                    if last_turn == Some(Tile::SouthToEast) {
                        intersects += 1;
                    }
                    last_turn = None;
                },
                Tile::NorthToEast => {
                    if last_turn == Some(Tile::SouthToWest) {
                        intersects += 1;
                    }
                    last_turn = None;
                },
                _ => (),
            }
        }
        intersects % 2 == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_loop() {
        let grid = Grid::parse("
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        ");

        assert_eq!(grid.find_start(), (1, 1));

        assert!(!grid.is_connected((1, 1), (1, 0)), "west");
        assert!(!grid.is_connected((1, 1), (0, 1)), "north");
        assert!(grid.is_connected((1, 1), (1, 2)), "east");
        assert!(grid.is_connected((1, 1), (2, 1)), "south");

        assert!(grid.is_connected((3, 3), (3, 2)), "west");
        assert!(grid.is_connected((3, 3), (2, 3)), "north");
        assert!(!grid.is_connected((3, 3), (3, 4)), "east");
        assert!(!grid.is_connected((3, 3), (4, 3)), "south");

        assert_eq!(grid.adjacent((0, 0)), vec![(1, 0), (0, 1)]);
        assert_eq!(grid.adjacent((1, 1)), vec![(0, 1), (2, 1), (1, 0), (1, 2)]);
        assert_eq!(grid.adjacent((4, 4)), vec![(3, 4), (4, 3)]);

        assert_eq!(grid.find_loop(), HashSet::from([(1, 1), (1, 2), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 1)]));
    }

    #[test]
    fn test_solve_part_1_simple_loop() {
        assert_eq!(solve_part_1(&Grid::parse("
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        ")), 4);
    }

    #[test]
    fn test_solve_part_1_complex_loop() {
        assert_eq!(solve_part_1(&Grid::parse("
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        ")), 8);
    }

    #[test]
    fn test_solve_part_2_simple_loop() {
        assert_eq!(solve_part_2(&Grid::parse("
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        ")), 4);
    }

    #[test]
    fn test_solve_part_2_narrow_loop() {
        assert_eq!(solve_part_2(&Grid::parse("
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........
        ")), 4);
    }

    #[test]
    fn test_solve_part_2_large_loop() {
        assert_eq!(solve_part_2(&Grid::parse("
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        ")), 8);
    }

    #[test]
    fn test_solve_part_2_loop_with_junk() {
        assert_eq!(solve_part_2(&Grid::parse("
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        ")), 10);
    }
}
