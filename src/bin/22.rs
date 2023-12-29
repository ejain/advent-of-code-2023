use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use itertools::Itertools;
use multimap::MultiMap;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/22.txt").unwrap();
    let mut snapshot = Snapshot::parse(&input);
    println!("Part 1: {}", solve_part_1(&mut snapshot));
    println!("Part 2: {}", solve_part_2(&mut snapshot));
}

fn solve_part_1(snapshot: &mut Snapshot) -> usize {
    let mut removable = 0;
    snapshot.settle();
    for brick in snapshot.cubes.keys().sorted() {
        let mut trial = snapshot.clone();
        trial.remove_brick(brick);
        if trial.settle() == 0 {
            removable += 1;
        }
    }
    removable
}

fn solve_part_2(snapshot: &mut Snapshot) -> usize {
    let mut moved = 0;
    snapshot.settle();
    for brick in snapshot.cubes.keys().sorted() {
        let mut trial = snapshot.clone();
        trial.remove_brick(brick);
        moved += trial.settle();
    }
    moved
}

type Cube = (usize, usize, usize);

type Brick = usize;

#[derive(Clone)]
struct Snapshot {
    cubes: MultiMap<Brick, Cube>,
    bricks: HashMap<Cube, Brick>,
}

impl Snapshot {
    fn parse(input: &str) -> Self {
        let mut snapshot = Self { cubes: MultiMap::new(), bricks: HashMap::new() };
        let mut brick_id = 0;
        let re = regex!(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)");
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
            if let Some(c) = re.captures(line) {
                let (_, [x1, y1, z1, x2, y2, z2]) = c.extract();
                let from = (x1.parse().unwrap(), y1.parse().unwrap(), z1.parse().unwrap());
                let to = (x2.parse().unwrap(), y2.parse().unwrap(), z2.parse().unwrap());
                let cubes = Self::get_cubes(&from, &to);
                for cube in cubes {
                    snapshot.cubes.insert(brick_id, cube);
                    snapshot.bricks.insert(cube, brick_id);
                }
                brick_id += 1;
            }
        }
        snapshot
    }

    fn get_cubes(from: &Cube, to: &Cube) -> HashSet<Cube> {
        let mut cubes = HashSet::new();
        if from.0 != to.0 {
            for x in from.0..=to.0 {
                cubes.insert((x, from.1, from.2));
            }
        } else if from.1 != to.1 {
            for y in from.1..=to.1 {
                cubes.insert((from.0, y, from.2));
            }
        } else if from.2 != to.2 {
            for z in from.2..=to.2 {
                cubes.insert((from.0, from.1, z));
            }
        } else {
            cubes.insert(*from);
        }
        cubes
    }

    fn get_base(cubes: &[Cube]) -> HashSet<Cube> {
        let min_z = cubes.iter().map(|cube| cube.2).min().unwrap();
        cubes.iter().filter(|&cube| cube.2 == min_z).cloned().collect()
    }

    fn get_clearance(&self, cube: &Cube) -> usize {
        let mut base = *cube;
        while base.2 > 1 {
            let next = (base.0, base.1, base.2 - 1);
            if self.bricks.contains_key(&next) {
                break;
            }
            base = next;
        }
        cube.2 - base.2
    }

    fn move_brick(&mut self, brick: &Brick, distance: usize) {
        if let Some(cubes) = self.cubes.get_vec_mut(brick) {
            cubes.iter_mut().sorted().for_each(|cube| {
                self.bricks.remove(cube);
                *cube = (cube.0, cube.1, cube.2 - distance);
                self.bricks.insert(*cube, *brick);
            });
        }
    }

    fn settle(&mut self) -> usize {
        let mut moved = HashSet::new();
        loop {
            let mut changed = false;
            let cubes = self.cubes.clone();
            for (brick, cubes) in &cubes {
                let d = Self::get_base(cubes).iter().map(|cube| self.get_clearance(cube)).min().unwrap_or(0);
                if d > 0 {
                    self.move_brick(brick, d);
                    moved.insert(*brick);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        moved.len()
    }

    fn remove_brick(&mut self, brick: &Brick) {
        self.cubes.remove(brick);
        self.bricks.retain(|_, b| b != brick );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case((1, 0, 1), (1, 0, 1), &[(1, 0, 1)]; "single-cube block")]
    #[test_case((0, 0, 2), (2, 0, 2), &[(0, 0, 2), (1, 0, 2), (2, 0, 2)]; "block along x-axis")]
    #[test_case((1, 0, 1), (1, 2, 1), &[(1, 0, 1), (1, 1, 1), (1, 2, 1)]; "block along y-axis")]
    #[test_case((1, 1, 8), (1, 1, 9), &[(1, 1, 8), (1, 1, 9)]; "block along z-axis")]
    fn test_get_cubes(from: Cube, to: Cube, expected: &[Cube]) {
        assert_eq!(Snapshot::get_cubes(&from, &to), HashSet::from_iter(expected.iter().cloned()));
    }

    #[test_case(&[(1, 0, 1), (1, 0, 1)], &[(1, 0, 1)]; "single-cube block")]
    #[test_case(&[(0, 0, 2), (1, 0, 2)], &[(0, 0, 2), (1, 0, 2)]; "block along x-axis")]
    #[test_case(&[(1, 0, 1), (1, 1, 1)], &[(1, 0, 1), (1, 1, 1)]; "block along y-axis")]
    #[test_case(&[(1, 1, 8), (1, 1, 9)], &[(1, 1, 8)]; "block along z-axis")]
    fn test_get_base(cubes: &[Cube], expected: &[Cube]) {
        assert_eq!(Snapshot::get_base(cubes), HashSet::from_iter(expected.iter().cloned()));
    }

    #[test]
    fn test_get_clearance() {
        let snapshot = Snapshot::parse("
            0,0,2~0,0,2
            1,1,1~1,1,1
            1,1,2~1,1,2
            1,1,5~1,1,5
        ");

        assert_eq!(snapshot.get_clearance(&(0, 0, 2)), 1);
        assert_eq!(snapshot.get_clearance(&(1, 1, 1)), 0);
        assert_eq!(snapshot.get_clearance(&(1, 1, 2)), 0);
        assert_eq!(snapshot.get_clearance(&(1, 1, 5)), 2);
    }

    #[test]
    fn test_move_brick() {
        let mut snapshot = Snapshot::parse("
            2,2,3~2,2,3
        ");

        snapshot.move_brick(&0, 2);
        assert_eq!(snapshot.cubes.get_vec(&0), Some(&vec![(2, 2, 1)]));
    }

    #[test]
    fn test_settle() {
        let mut snapshot = Snapshot::parse("
            0,0,1~0,0,1
            0,0,4~0,0,4
        ");

        assert_eq!(snapshot.settle(), 1);
        assert_eq!(snapshot.settle(), 0);
        assert_eq!(snapshot.cubes.get_vec(&0), Some(&vec![(0, 0, 1)]));
        assert_eq!(snapshot.cubes.get_vec(&1), Some(&vec![(0, 0, 2)]));
    }

    #[test]
    fn test_remove_brick() {
        let mut snapshot = Snapshot::parse("
            0,0,1~0,0,1
            0,0,4~0,0,4
        ");

        snapshot.remove_brick(&0);
        assert_eq!(snapshot.cubes.get_vec(&0), None);
        assert_eq!(snapshot.cubes.get_vec(&1), Some(&vec![(0, 0, 4)]));
    }

    #[test]
    fn test_solve_part_1() {
        let mut snapshot = Snapshot::parse("
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        ");

        assert_eq!(solve_part_1(&mut snapshot), 5);
    }

    #[test]
    fn test_solve_part_2() {
        let mut snapshot = Snapshot::parse("
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        ");

        assert_eq!(solve_part_2(&mut snapshot), 7);
    }
}
