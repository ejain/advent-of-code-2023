use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/17.txt").unwrap();
    let city = City::parse(&input);
    println!("Part 1: {}", solve_part_1(&city));
}

fn solve_part_1(city: &City) -> u32 {
    city.find_route()
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn inverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

type Block = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Route {
    block: Block,
    heading: Heading,
    distance: u32,
    heat_loss: u32,
}

impl Route {
    fn new(block: Block, heading: Heading, distance: u32, heat_loss: u32) -> Self {
        Self { block, heading, distance, heat_loss }
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct City {
    blocks: Vec<Vec<u32>>,
}

impl City {
    fn parse(input: &str) -> City {
        let blocks = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Self { blocks }
    }

    fn find_route(&self) -> u32 {
        let max_distance: u32 = 3;
        let from = (0, 0);
        let to = (self.blocks.len() - 1, self.blocks[0].len() - 1);

        let mut lowest_heat_loss = u32::MAX;
        let mut seen = HashMap::new();
        let mut routes = BinaryHeap::new();
        routes.push(Route::new(from, Heading::Right, 1, 0));
        routes.push(Route::new(from, Heading::Down, 1, 0));

        while let Some(Route { block, heading, distance, heat_loss }) = routes.pop() {
            if heat_loss >= lowest_heat_loss || seen.get(&(block, heading, distance)).is_some_and(|&lowest_heat_loss| heat_loss >= lowest_heat_loss) {
                continue;
            }
            seen.insert((block, heading, distance), heat_loss);

            if let Some(next_block) = self.next(&block, &heading) {
                let next_heat_loss = heat_loss + self.loss_at(&next_block);
                if next_heat_loss >= lowest_heat_loss || seen.get(&(next_block, heading, distance)).is_some_and(|&prev_loss| next_heat_loss >= prev_loss) {
                    continue;
                }
                if next_block == to {
                    lowest_heat_loss = next_heat_loss;
                    continue;
                }

                for next_heading in [Heading::Up, Heading::Down, Heading::Left, Heading::Right] {
                    if next_heading != heading.inverse() {
                        let mut next_distance = 1;
                        if next_heading == heading {
                            if distance >= max_distance {
                                continue;
                            }
                            next_distance = distance + 1;
                        }
                        routes.push(Route::new(next_block, next_heading, next_distance, next_heat_loss));

                    }
                }
            }
        }

        lowest_heat_loss
    }

    fn loss_at(&self, block: &Block) -> u32 {
        self.blocks[block.0][block.1]
    }

    fn next(&self, b: &Block, h: &Heading) -> Option<Block> {
        match h {
            Heading::Up => if b.0 > 0 { Some((b.0 - 1, b.1)) } else { None },
            Heading::Down => if b.0 + 1 < self.blocks.len() { Some((b.0 + 1, b.1)) } else { None },
            Heading::Left => if b.1 > 0 { Some((b.0, b.1 - 1)) } else { None },
            Heading::Right => if b.1 + 1 < self.blocks[0].len() { Some((b.0, b.1 + 1)) } else { None },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        let city = City::parse(r"
            123
            456
            789
        ");

        assert_eq!(city.next(&(0, 0), &Heading::Up), None);
        assert_eq!(city.next(&(0, 0), &Heading::Down), Some((1, 0)));
        assert_eq!(city.next(&(0, 0), &Heading::Left), None);
        assert_eq!(city.next(&(0, 0), &Heading::Right), Some((0, 1)));

        assert_eq!(city.next(&(1, 1), &Heading::Up), Some((0, 1)));
        assert_eq!(city.next(&(1, 1), &Heading::Down), Some((2, 1)));
        assert_eq!(city.next(&(1, 1), &Heading::Left), Some((1, 0)));
        assert_eq!(city.next(&(1, 1), &Heading::Right), Some((1, 2)));

        assert_eq!(city.next(&(2, 2), &Heading::Up), Some((1, 2)));
        assert_eq!(city.next(&(2, 2), &Heading::Down), None);
        assert_eq!(city.next(&(2, 2), &Heading::Left), Some((2, 1)));
        assert_eq!(city.next(&(2, 2), &Heading::Right), None);
    }

    #[test]
    fn test_find_route() {
        let city = City::parse(r"
            11111
            22221
            22221
        ");
        assert_eq!(city.find_route(), 7);
    }

    #[test]
    fn test_solve_part_1() {
        let city = City::parse(r"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        ");
        assert_eq!(solve_part_1(&city), 102);
    }
}
