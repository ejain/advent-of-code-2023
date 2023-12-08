use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Add;

fn main() {
    let input = read_to_string("data/03.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> u32 {
    let mut sum = 0;
    let schematic = Schematic::parse(input);
    for row in 0..schematic.rows {
        let mut value = 0;
        let mut adjacent_to_symbol = false;
        for col in 0..schematic.cols {
            if schematic.is_digit(row, col) {
                value = value * 10 + schematic.value_at(row, col);
                if !adjacent_to_symbol {
                    adjacent_to_symbol = schematic.is_adjacent_to_symbol(row, col);
                }
            } else {
                if adjacent_to_symbol && value > 0 {
                    sum += value;
                }
                value = 0;
                adjacent_to_symbol = false;
            }
        }
        if adjacent_to_symbol && value > 0 {
            sum += value;
        }
    }
    sum
}

fn solve_part_2(input: &str) -> u32 {
    let schematic = Schematic::parse(input);
    let mut gears = HashMap::new();

    for row in 0..schematic.rows {
        let mut value = 0;
        let mut gear: Option<(usize, usize)> = None;
        for col in 0..schematic.cols {
            if schematic.is_digit(row, col) {
                value = value * 10 + schematic.value_at(row, col);
                if gear.is_none() {
                    gear = schematic.find_adjacent_gear(row, col);
                }
            } else {
                if value > 0 {
                    if let Some(gear) = gear {
                        gears.entry(gear).or_insert_with(Vec::new).push(value);
                    }
                }
                value = 0;
                gear = None;
            }
        }
        if value > 0 {
            if let Some(gear) = gear {
                gears.entry(gear).or_insert_with(Vec::new).push(value);
            }
        }
    }

    let mut sum = 0;
    for (_, values) in gears {
        if values.len() == 2 {
            sum += values[0] * values[1];
        }
    }
    sum
}

struct Schematic {
    rows: usize,
    cols: usize,
    values: Vec<char>,
}

impl Schematic {
    fn parse(input: &str) -> Schematic {
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        let mut values = String::new();
        for mut line in input.lines() {
            line = line.trim();
            if !line.is_empty() {
                rows += 1;
                cols = line.len();
                values = values.add(line);
            }
        }
        Schematic { rows, cols, values: values.chars().collect() }
    }

    fn get(&self, row: usize, col: usize) -> char {
        assert!(self.contains(row, col), "({}, {}) is out of bounds", row, col);
        self.values[row * self.cols + col]
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        (0..self.rows).contains(&row) && (0..self.cols).contains(&col)
    }

    fn value_at(&self, row: usize, col: usize) -> u32 {
        let value = self.get(row, col);
        if value.is_ascii_digit() {
            value.to_digit(10).unwrap()
        } else {
            0
        }
    }

    fn is_digit(&self, row: usize, col: usize) -> bool {
        let value = self.get(row, col);
        value.is_ascii_digit()
    }

    fn is_adjacent_to_symbol(&self, row: usize, col: usize) -> bool {
        (row > 0 && col > 0 && self.is_symbol(row - 1, col - 1)) || // up & left
        (row > 0 && self.is_symbol(row - 1, col)) || // up
        (row > 0 && col + 1 < self.cols && self.is_symbol(row - 1, col + 1)) || // up & right
        (col > 0 && self.is_symbol(row, col - 1)) || // left
        (col + 1 < self.cols && self.is_symbol(row, col + 1)) || // right
        (row + 1 < self.rows && col > 0 && self.is_symbol(row + 1, col - 1)) || // down & left
        (row + 1 < self.rows && self.is_symbol(row + 1, col)) || // down
        (row + 1 < self.rows && col + 1 < self.cols && self.is_symbol(row + 1, col + 1)) // down & right
    }

    fn is_symbol(&self, row: usize, col: usize) -> bool {
        let value = self.get(row, col);
        value != '.' && !value.is_ascii_digit()
    }

    fn find_adjacent_gear(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row > 0 && col > 0 && self.is_gear(row - 1, col - 1) {
            Some((row - 1, col - 1))
        } else if row > 0 && self.is_gear(row - 1, col) {
            Some((row - 1, col))
        } else if row > 0 && col + 1 < self.cols && self.is_gear(row - 1, col + 1) {
            Some((row - 1, col + 1))
        } else if col > 0 && self.is_gear(row, col - 1) {
            Some((row, col - 1))
        } else if col + 1 < self.cols && self.is_gear(row, col + 1) {
            Some((row, col + 1))
        } else if row + 1 < self.rows && col > 0 && self.is_gear(row + 1, col - 1) {
            Some((row + 1, col - 1))
        } else if row + 1 < self.rows && self.is_gear(row + 1, col) {
            Some((row + 1, col))
        } else if row + 1 < self.rows && col + 1 < self.cols && self.is_gear(row + 1, col + 1) {
            Some((row + 1, col + 1))
        } else {
            None
        }
    }

    fn is_gear(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == '*'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let schematic = Schematic::parse("
            1234
            5.@6
            7890
        ");
        assert_eq!(schematic.rows, 3, "rows");
        assert_eq!(schematic.cols, 4, "columns");

        assert_eq!(schematic.get(0, 0), '1', "(0, 0))");
        assert_eq!(schematic.get(0, 3), '4', "(0, 3))");
        assert_eq!(schematic.get(2, 2), '9', "(2, 2))");

        assert_eq!(schematic.value_at(0, 0), 1, "value of '1'");
        assert_eq!(schematic.value_at(1, 1), 0, "value of ','");
        assert_eq!(schematic.value_at(1, 2), 0, "value of '@'");

        assert_eq!(schematic.is_digit(0, 0), true, "'1' is a digit)");
        assert_eq!(schematic.is_digit(1, 1), false, "'.' is a digit)");
        assert_eq!(schematic.is_digit(1, 2), false, "'@' is a digit)");

        assert_eq!(schematic.is_symbol(0, 0), false, "'1' is a symbol)");
        assert_eq!(schematic.is_symbol(1, 1), false, "`.` is a symbol)");
        assert_eq!(schematic.is_symbol(1, 2), true, "'@' is a symbol)");

        assert_eq!(schematic.is_adjacent_to_symbol(0, 0), false, "'1' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(0, 1), true, "'2' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(0, 2), true, "'3' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(1, 0), false, "'5' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(1, 3), true, "'6' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(2, 0), false, "'7' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(2, 1), true, "'8' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(2, 2), true, "'9' is adjacent to a symbol");
        assert_eq!(schematic.is_adjacent_to_symbol(2, 3), true, "'0' is adjacent to a symbol");
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "), 4361);
    }

    #[test]
    fn test_find_adjacent_gear() {
        let schematic = Schematic::parse("
            .....
            ..*..
            .....
        ");
        assert_eq!(schematic.find_adjacent_gear(0, 0), None, "gear adjacent to (0, 0)");
        assert_eq!(schematic.find_adjacent_gear(0, 1), Some((1, 2)), "gear adjacent to (0, 1)");
        assert_eq!(schematic.find_adjacent_gear(0, 2), Some((1, 2)), "gear adjacent to (0, 2)");
        assert_eq!(schematic.find_adjacent_gear(0, 3), Some((1, 2)), "gear adjacent to (0, 3)");
        assert_eq!(schematic.find_adjacent_gear(0, 4), None, "gear adjacent to (0, 4)");

        assert_eq!(schematic.find_adjacent_gear(1, 0), None, "gear adjacent to (1, 0)");
        assert_eq!(schematic.find_adjacent_gear(1, 1), Some((1, 2)), "gear adjacent to (1, 1)");
        assert_eq!(schematic.find_adjacent_gear(1, 2), None, "gear adjacent to (1, 2)");
        assert_eq!(schematic.find_adjacent_gear(1, 3), Some((1, 2)), "gear adjacent to (1, 3)");
        assert_eq!(schematic.find_adjacent_gear(1, 4), None, "gear adjacent to (1, 4)");

        assert_eq!(schematic.find_adjacent_gear(2, 0), None, "gear adjacent to (2, 0)");
        assert_eq!(schematic.find_adjacent_gear(2, 1), Some((1, 2)), "gear adjacent to (2, 1)");
        assert_eq!(schematic.find_adjacent_gear(2, 2), Some((1, 2)), "gear adjacent to (2, 2)");
        assert_eq!(schematic.find_adjacent_gear(2, 3), Some((1, 2)), "gear adjacent to (2, 3)");
        assert_eq!(schematic.find_adjacent_gear(2, 4), None, "gear adjacent to (2, 4)");
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2("
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "), 467835);
    }
}
