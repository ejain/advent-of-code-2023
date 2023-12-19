use std::cmp::max;
use std::fs::read_to_string;
use multimap::MultiMap;

fn main() {
    let input = read_to_string("data/16.txt").unwrap();
    let mut grid = Grid::parse(&input);
    println!("Part 1: {}", solve_part_1(&mut grid));
    println!("Part 2: {}", solve_part_2(&mut grid));
}

fn solve_part_1(grid: &mut Grid) -> u32 {
    grid.illuminate()
}

fn solve_part_2(grid: &mut Grid) -> u32 {
    let mut max_energized = 0;
    for row in 0..grid.tiles.len() {
        max_energized = max(max_energized, grid.illuminate_from((row, 0),  Direction::Right));
        max_energized = max(max_energized, grid.illuminate_from((grid.tiles[0].len() - 1, row), Direction::Left));
    }
    for col in 0..grid.tiles[0].len() {
        max_energized = max(max_energized, grid.illuminate_from((0, col), Direction::Down));
        max_energized = max(max_energized, grid.illuminate_from((grid.tiles.len() - 1, col), Direction::Up));
    }
    max_energized
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    ReflectUp,
    ReflectDown,
    SplitVertical,
    SplitHorizontal,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::ReflectUp,
            '\\' => Self::ReflectDown,
            '|' => Self::SplitVertical,
            '-' => Self::SplitHorizontal,
            _ => panic!("invalid tile <{}>", c),
        }
    }

    fn next_direction(&self, from: &Direction) -> Vec<Direction> {
        match self {
            Self::Empty => vec![*from],
            Self::ReflectUp => match from {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            Self::ReflectDown => match from {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            Self::SplitVertical => match from {
                Direction::Up | Direction::Down => vec![*from],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            Self::SplitHorizontal => match from {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![*from],
            },
        }
    }
}


type Point = (usize, usize);

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let tiles = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();
        Grid { tiles }
    }

    fn illuminate(&mut self) -> u32 {
        self.illuminate_from((0, 0), Direction::Right)
    }

    fn illuminate_from(&mut self, initial_position: Point, initial_direction: Direction) -> u32 {
        let mut illuminated = MultiMap::new();
        let mut beams: Vec<(Point, Direction)> = Vec::new();
        for initial_direction in self.tile_at(initial_position).next_direction(&initial_direction) {
            beams.push((initial_position, initial_direction));
        }
        while !beams.is_empty() {
            let (current_position, current_direction) = beams.remove(beams.len() - 1);
            illuminated.insert(current_position, current_direction);
            if let Some(next_position) = self.next_position(&current_position, &current_direction) {
                for next_direction in self.tile_at(next_position).next_direction(&current_direction) {
                    if !illuminated.get_vec(&next_position).unwrap_or(&vec![]).contains(&next_direction) {
                        beams.push((next_position, next_direction));
                    }
                }
            }
        }
        illuminated.keys().len() as u32
    }

    fn tile_at(&self, p: Point) -> Tile {
        self.tiles[p.0][p.1]
    }

    fn next_position(&self, from: &Point, dir: &Direction) -> Option<Point> {
        match dir {
            Direction::Up => if from.0 > 0 { Some((from.0 - 1, from.1)) } else { None },
            Direction::Down => if from.0 + 1 < self.tiles.len() { Some((from.0 + 1, from.1)) } else { None },
            Direction::Left => if from.1 > 0 { Some((from.0, from.1 - 1)) } else { None },
            Direction::Right => if from.1 + 1 < self.tiles[0].len() { Some((from.0, from.1 + 1)) } else { None },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deflect_beam() {
        let mut grid = Grid::parse(r"
            \/\
            ...
            \./
        ");
        assert_eq!(grid.illuminate(), 9);
    }

    #[test]
    fn test_split_beam() {
        let mut grid = Grid::parse(r"
            ..|
            -.|
            |.-
        ");
        assert_eq!(grid.illuminate(), 9);
    }

    #[test]
    fn test_solve_part_1() {
        let mut grid = Grid::parse(r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....\
        ");
        assert_eq!(solve_part_1(&mut grid), 46);
    }

    #[test]
    fn test_solve_part_2() {
        let mut grid = Grid::parse(r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....\
        ");
        assert_eq!(solve_part_2(&mut grid), 51);
    }
}
