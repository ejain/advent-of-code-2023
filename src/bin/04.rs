use std::cmp::min;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/04.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> u32 {
    input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Card::parse)
        .map(|card| card.score())
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Card::parse)
        .collect();
    let matches: Vec<u32> = cards.iter().map(Card::matches).collect();
    let mut counts: Vec<u32> = iter::repeat(1).take(cards.len()).collect();
    for i in 0..cards.len() {
        for j in (i + 1)..min(i + 1 + matches[i] as usize, counts.len()) {
            counts[j] += counts[i];
        }
    }
    counts.iter().sum()
}

struct Card {
    expected: HashSet<u32>,
    actual: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Card {
        let re = regex!(r"Card\s+\d+:((?:\s+\d+)+) \|((?:\s+\d+)+)");
        let c = re.captures(line).unwrap();
        Card {
            expected: c[1].split_ascii_whitespace().map(|s| s.parse().unwrap()).collect(),
            actual: c[2].split_ascii_whitespace().map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches > 0 {
            u32::pow(2, matches - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> u32 {
        self.actual.iter().filter(|n| self.expected.contains(n)).count() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matches_and_score() {
        let card = Card {
            expected: HashSet::from([2, 4, 6, 8]),
            actual: Vec::from([1, 2, 3, 4, 5, 6, 7])
        };
        assert_eq!(card.matches(), 3, "matches");
        assert_eq!(card.score(), 4, "score");
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "), 13);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2("
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "), 30);
    }
}
