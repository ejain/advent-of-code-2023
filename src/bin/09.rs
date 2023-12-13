use std::fs::read_to_string;

fn main() {
    let input = read_to_string("data/09.txt").unwrap();
    let report = parse(&input);
    println!("Part 1: {}", solve_part_1(&report));
    println!("Part 2: {}", solve_part_2(&report));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|tokens| tokens.iter().map(|token| token.parse().unwrap()).collect())
        .collect()
}

fn solve_part_1(report: &[Vec<i32>]) -> i32 {
    report.iter()
        .map(|values| extrapolate(values))
        .sum()
}

fn solve_part_2(report: &[Vec<i32>]) -> i32 {
    report.iter()
        .map(|values| values.iter().rev().cloned().collect::<Vec<i32>>())
        .map(|values| extrapolate(&values))
        .sum()
}

fn extrapolate(values: &[i32]) -> i32 {
    if values.iter().all(|value| *value == 0) {
        return 0
    }
    let mut deltas = Vec::new();
    for i in 0..values.len() - 1 {
        deltas.push(values[i + 1] - values[i]);
    }
    values.last().unwrap() + extrapolate(&deltas)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_solve_part_1() {
        let report = parse("
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ");
        assert_eq!(solve_part_1(&report), 114);
    }

    #[test]
    fn test_solve_part_2() {
        let report = parse("
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ");
        assert_eq!(solve_part_2(&report), 2);
    }
}
