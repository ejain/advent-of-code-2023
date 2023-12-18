use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/15.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .map(|s| hash(s) as u32)
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let mut path = Path::new();
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<Operation>().unwrap())
        .for_each(|op| op.process(&mut path));
    path.get_focusing_power()
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

#[derive(Debug, PartialEq)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Lens {
    fn new(label: &str, focal_length: u32) -> Self {
        Self { label: label.to_string(), focal_length }
    }
}

struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    fn insert(&mut self, label: &str, focal_length: u32) {
        if let Some((i, _)) = self.lenses.iter().find_position(|&lens| lens.label == label) {
            self.lenses[i].focal_length = focal_length;
        } else {
            self.lenses.push(Lens::new(label, focal_length))
        }
    }

    fn remove(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn get_focusing_power(&self) -> u32 {
        let mut focusing_power = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            focusing_power += (i + 1) as u32 * lens.focal_length;
        }
        focusing_power
    }
}

struct Path {
    boxes: Vec<Box>,
}

impl Path {
    fn new() -> Self {
        Self { boxes: (0..256).map(|_| Box::new()).collect() }
    }

    fn get_focusing_power(&self) -> u32 {
        let mut focusing_power = 0;
        for (i, b) in self.boxes.iter().enumerate() {
            focusing_power += (i + 1) as u32 * b.get_focusing_power();
        }
        focusing_power
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Insert(String, u32),
    Remove(String),
}

impl Operation {
    fn process(&self, path: &mut Path) {
        match self {
            Self::Insert(label, focal_length) => path.boxes[hash(label)].insert(label, *focal_length),
            Self::Remove(label) => path.boxes[hash(label)].remove(label),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"([a-z]+)(=|-)(\d*)");
        if let Some(cap) = re.captures(s) {
            let label = cap.get(1).map(|m| m.as_str()).unwrap();
            let op = cap.get(2).map(|m| m.as_str()).unwrap();
            let focal_length = cap.get(3).map(|m| m.as_str().parse::<u32>().unwrap_or(0)).unwrap();
            match op {
                "=" => Ok(Self::Insert(label.to_string(), focal_length)),
                "-" => Ok(Self::Remove(label.to_string())),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_solve_part_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve_part_1(&input), 1320);
    }

    #[test]
    fn test_process_operations() {
        let mut path = Path::new();

        Operation::Insert("rn".to_string(), 1).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 1);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));

        Operation::Remove("cm".to_string()).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 1);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));

        Operation::Insert("qp".to_string(), 3).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 1);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));
        assert_eq!(path.boxes[1].lenses.len(), 1);
        assert_eq!(path.boxes[1].lenses[0], Lens::new("qp", 3));

        Operation::Insert("cm".to_string(), 2).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 2);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));
        assert_eq!(path.boxes[0].lenses[1], Lens::new("cm", 2));
        assert_eq!(path.boxes[1].lenses.len(), 1);
        assert_eq!(path.boxes[1].lenses[0], Lens::new("qp", 3));

        Operation::Remove("qp".to_string()).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 2);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));
        assert_eq!(path.boxes[0].lenses[1], Lens::new("cm", 2));
        assert_eq!(path.boxes[1].lenses.len(), 0);

        Operation::Insert("pc".to_string(), 4).process(&mut path);
        assert_eq!(path.boxes[0].lenses.len(), 2);
        assert_eq!(path.boxes[0].lenses[0], Lens::new("rn", 1));
        assert_eq!(path.boxes[0].lenses[1], Lens::new("cm", 2));
        assert_eq!(path.boxes[1].lenses.len(), 0);
        assert_eq!(path.boxes[2].lenses.len(), 0);
        assert_eq!(path.boxes[3].lenses.len(), 1);
        assert_eq!(path.boxes[3].lenses[0], Lens::new("pc", 4));
    }

    #[test]
    fn test_get_focusing_power() {
        let mut b = Box::new();
        b.insert("rn", 1);
        b.insert("cm", 2);
        assert_eq!(b.get_focusing_power(), 5);
    }

    #[test]
    fn test_solve_part_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve_part_2(&input), 145);
    }
}
