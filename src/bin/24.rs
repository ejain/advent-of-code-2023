use std::fs::read_to_string;
use std::ops::RangeInclusive;
use regex_macro::regex;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2, Vector3};

fn main() {
    let input = read_to_string("data/24.txt").unwrap();
    let trajectories = parse(&input);
    println!("Part 1: {}", solve_part_1(&trajectories, &(200_000_000_000_000.0..=400_000_000_000_000.0)));

}

fn solve_part_1(trajectories: &[Trajectory], range: &RangeInclusive<f64>) -> usize {
    trajectories.iter().combinations(2).filter(|c| c[0].intersects(c[1], range)).count()
}

fn parse(input: &str) -> Vec<Trajectory> {
    input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Trajectory::parse)
        .collect()
}

#[derive(Debug)]
struct Trajectory {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Trajectory {
    fn parse(line: &str) -> Self {
        let re = regex!(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+(\-?\d+),\s+(\-?\d+),\s+(\-?\d+)");
        if let Some(c) = re.captures(line) {
            let (_, [px, py, pz, vx, vy, vz]) = c.extract();
            Self {
                position: Vector3::new(px.parse().unwrap(), py.parse().unwrap(), pz.parse().unwrap()),
                velocity: Vector3::new(vx.parse().unwrap(), vy.parse().unwrap(), vz.parse().unwrap())
            }
        } else {
            panic!("can't parse <{}>", line);
        }
    }

    fn intersects(&self, other: &Trajectory, range: &RangeInclusive<f64>) -> bool {
        let coefficients = Matrix2::new(
            self.velocity.x, -other.velocity.x,
            self.velocity.y, -other.velocity.y,
        );
        let rhs = Vector2::new(
            other.position.x - self.position.x,
            other.position.y - self.position.y,
        );
        if let Some(solution) = coefficients.lu().solve(&rhs) {
            let x = self.position.x + solution.x * self.velocity.x;
            let y = self.position.y + solution.x * self.velocity.y;
            return solution.x >= 0.0 && solution.y >= 0.0 && range.contains(&x) && range.contains(&y)
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let trajectories = parse("
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        ");

        assert_eq!(solve_part_1(&trajectories, &(7.0..=27.0)), 2);
    }
}
