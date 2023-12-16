use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use itertools::Itertools;

fn main() {
    let input = read_to_string("data/13.txt").unwrap();
    let areas = parse(&input);
    println!("Part 1: {}", solve_part_1(&areas));
    println!("Part 2: {}", solve_part_2(&areas));
}

fn parse(input: &str) -> Vec<Area> {
    input
        .replace('\r', "")
        .split("\n\n")
        .map(str::trim)
        .map(Area::parse)
        .collect()
}

fn solve_part_1(areas: &[Area]) -> u32 {
    areas.iter()
        .flat_map(|area| area.summarize())
        .sum()
}

fn solve_part_2(areas: &[Area]) -> u32 {
    let mut sum = 0;
    'area: for area in areas {
        let original_score = area.summarize().iter().sum::<u32>();
        for row in 0..area.values.len() {
            for col in 0..area.values[row].len() {
                let scores = area.clean(row, col).summarize().iter().cloned()
                    .filter(|&score| score != original_score)
                    .collect::<HashSet<u32>>();
                if !scores.is_empty() {
                    assert_eq!(scores.len(), 1);
                    sum += scores.iter().sum::<u32>();
                    continue 'area;
                }
            }
        }
        panic!("no score for {:?}", area.values.iter().map(|row| row.iter().join("")).join("\n"));
    }
    sum
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Area {
    values: Vec<Vec<char>>,
    row_hashes: Vec<u64>,
    col_hashes: Vec<u64>,
}

impl Area {
    fn parse(input: &str) -> Area {
        let mut row_hashes = Vec::new();
        let mut col_hashes = Vec::new();
        let lines: Vec<Vec<char>> = input.lines()
            .map(str::trim)
            .filter(|&line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        assert!(!lines.is_empty());
        for line in &lines {
            let mut hasher = DefaultHasher::new();
            line.hash(&mut hasher);
            row_hashes.push(hasher.finish())
        }
        let num_cols: usize = lines[0].len();
        for col in 0..num_cols {
            let mut hasher = DefaultHasher::new();
            for line in &lines {
                assert_eq!(line.len(), num_cols, "malformed area {:?}", input);
                line[col].hash(&mut hasher);
            }
            col_hashes.push(hasher.finish())
        }
        Self { values: lines, row_hashes, col_hashes }
    }

    fn summarize(&self) -> HashSet<u32> {
        self.count_mirrored_rows().iter()
            .map(|&count| count * 100)
            .chain(self.count_mirrored_cols())
            .collect()
    }

    fn count_mirrored_rows(&self) -> Vec<u32> {
        Self::count_mirrored(&self.row_hashes)
    }

    fn count_mirrored_cols(&self) -> Vec<u32> {
        Self::count_mirrored(&self.col_hashes)
    }

    fn count_mirrored(values: &[u64]) -> Vec<u32> {
        let mut scores = Vec::new();
        'i: for i in 0..(values.len() - 1) {
            let j = i + 1;
            for offset in 0..min(i + 1, values.len() - j) {
                if values[i - offset] != values[j + offset] {
                    continue 'i;
                }
            }
            scores.push(j as u32);
        }
        scores
    }

    fn clean(&self, row: usize, col: usize) -> Area {
        let mut values = self.values.to_vec();
        values[row][col] = if values[row][col] == '.' { '#' } else { '.' };

        let mut row_hashes = self.row_hashes.to_vec();
        let mut hasher = DefaultHasher::new();
        values[row].hash(&mut hasher);
        row_hashes[row] = hasher.finish();

        let mut col_hashes = self.col_hashes.to_vec();
        let mut hasher = DefaultHasher::new();
        for line in &values {
            line[col].hash(&mut hasher);
        }
        col_hashes[col] = hasher.finish();

        Self { values, row_hashes, col_hashes }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let area = Area::parse("
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        ");
        assert_eq!(area.row_hashes.len(), 7);
        assert_eq!(area.row_hashes[2], area.row_hashes[3]);
        assert_eq!(area.col_hashes.len(), 9);
        assert_eq!(area.col_hashes[4], area.col_hashes[5]);
    }

    #[test]
    fn test_count_mirrored_cols() {
        let area = Area::parse("
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        ");
        assert_eq!(area.count_mirrored_cols(), vec![5]);
    }

    #[test]
    fn test_count_mirrored_rows() {
        let area = Area::parse("
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ");
        assert_eq!(area.count_mirrored_rows(), vec![4]);
    }

    #[test]
    fn test_solve_part_1() {
        let areas = parse("
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ");
        assert_eq!(solve_part_1(&areas), 405);
    }

    #[test]
    fn test_clean_1() {
        let area = Area::parse("
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        ").clean(0, 0);
        assert_eq!(area.count_mirrored_rows(), vec![3]);
        assert_eq!(area.count_mirrored_cols(), vec![5]);
    }

    #[test]
    fn test_clean_2() {
        let area = Area::parse("
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ").clean(1, 4);
        assert_eq!(area.count_mirrored_rows(), vec![1]);
        assert_eq!(area.count_mirrored_cols(), vec![]);
    }

    #[test]
    fn test_clean_3() {
        let area = Area::parse("
            #...##...#..#.#..
            #...##...#..#.#..
            .#.#...#.#....#..
            #.#..#.##..##.#.#
            #..###.#..#..#...
            ###.#####.##.#.#.
            ##############...
            #.....#######..#.
            ..##..##.####..##
            #.####..##.#..#.#
            #.####..##.#..#.#
            ..##..##.####..##
            #.....#######..#.
            ##############..#
            ###.#####.##.#.#.
            #..###.#..#..#...
            #.#..#.##..##.#.#
        ").clean(6, 16);
        assert_eq!(area.count_mirrored_rows(), vec![1, 10]);
        assert_eq!(area.count_mirrored_cols(), vec![]);
    }

    #[test]
    fn test_clean_4() {
        let area = Area::parse("
            ....###.##.###.
            #.#####.#...#.#
            #..#.#...##.###
            ..#####..##.##.
            ..#####..##.##.
            #..#.#..###.###
            #.#####.#...#.#
            ....###.##.###.
            ...###.#.###...
            ...##...#..#...
            .####.#....###.
            .####.#....###.
            ...##...#..#...
        ").clean(5, 8);
        assert_eq!(area.count_mirrored_rows(), vec![4, 11]);
        assert_eq!(area.count_mirrored_cols(), vec![]);
    }

    #[test]
    fn test_solve_part_2() {
        let areas = parse("
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ");
        assert_eq!(solve_part_2(&areas), 400);
    }
}
