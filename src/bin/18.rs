use std::fs::read_to_string;
use std::str::FromStr;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/18.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> i64 {
    let plan = Plan::parse(input);
    plan.get_area()
}

fn solve_part_2(input: &str) -> i64 {
    let plan = Plan::parse_from_hex(input);
    plan.get_area()
}

struct Plan {
    steps: Vec<Step>,
}

impl Plan {
    fn parse(input: &str) -> Self {
        let steps = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Step::parse)
            .collect();
        Self { steps }
    }

    fn parse_from_hex(input: &str) -> Self {
        let steps = input.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Step::parse_from_hex)
            .collect();
        Self { steps }
    }

    fn get_area(&self) -> i64 {
        let mut area = 0;
        let mut distance = 0;
        let mut current = (0, 0);
        for step in &self.steps {
            distance += step.distance;
            match step.direction {
                Direction::Up => {
                    area -= current.1 * step.distance;
                    current = (current.0 - step.distance, current.1)
                },
                Direction::Down => {
                    area += current.1 * step.distance;
                    current = (current.0 + step.distance, current.1)
                },
                Direction::Left => {
                    current = (current.0, current.1 - step.distance)
                },
                Direction::Right => {
                    current = (current.0, current.1 + step.distance)
                },
            }
        }
        area + distance / 2 + 1
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Step {
    direction: Direction,
    distance: i64,
}

impl Step {
    fn parse(line: &str) -> Self {
        let re = regex!(r"([UDRL]) (\d+) \(#[\p{Hex_Digit}]+\)");
        if let Some(captures) = re.captures(line) {
            let direction = captures.get(1).unwrap().as_str().parse().unwrap();
            let distance = captures.get(2).unwrap().as_str().parse().unwrap();
            Self { direction, distance }
        } else {
            panic!("can't parse step <{}>", line);
        }
    }

    fn parse_from_hex(line: &str) -> Self {
        let re = regex!(r"[UDRL] \d+ \(#([\p{Hex_Digit}]{5})([0-3])\)");
        if let Some(captures) = re.captures(line) {
            let distance = i64::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
            let direction = captures.get(2).unwrap().as_str().parse().unwrap();
            Self { direction, distance }
        } else {
            panic!("can't parse step <{}>", line);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "3" => Ok(Self::Up),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "R" | "0" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let plan = Plan::parse(r"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        ");
        assert_eq!(plan.steps.len(), 14);
        assert_eq!(plan.steps[0], Step { direction: Direction::Right, distance: 6 });
        assert_eq!(plan.steps[1], Step { direction: Direction::Down, distance: 5 });
        assert_eq!(plan.steps[2], Step { direction: Direction::Left, distance: 2 });
        assert_eq!(plan.get_area(), 62);
    }

    #[test]
    fn test_solve_part_2() {
        let plan = Plan::parse_from_hex(r"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        ");
        assert_eq!(plan.steps.len(), 14);
        assert_eq!(plan.steps[0], Step { direction: Direction::Right, distance: 461_937 });
        assert_eq!(plan.steps[1], Step { direction: Direction::Down, distance: 56_407 });
        assert_eq!(plan.steps[2], Step { direction: Direction::Right, distance: 356_671 });
        assert_eq!(plan.get_area(), 952_408_144_115);
    }
}
