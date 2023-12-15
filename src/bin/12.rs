use std::cmp::min;
use std::fs::read_to_string;
use itertools::Itertools;
use memoize::memoize;

fn main() {
    let input = read_to_string("data/12.txt").unwrap();
    let records = parse(&input);
    println!("Part 1: {}", solve_part_1(&records));
    println!("Part 2: {}", solve_part_2(&records));
}

fn parse(input: &str) -> Vec<Record> {
    input.lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let tokens = line.split_ascii_whitespace().collect_vec();
            let conditions = tokens[0].chars().collect();
            let groups = tokens[1].split(',').map(|s| s.parse().unwrap()).collect();
            Record { conditions, groups }
        })
        .collect()
}

fn solve_part_1(records: &[Record]) -> u64 {
    records.iter()
        .cloned()
        .map(count)
        .sum()
}

fn solve_part_2(records: &[Record]) -> u64 {
    records.iter()
        .map(|rec| rec.unfold())
        .map(count)
        .sum()
}

const CONDITION_OPERATIONAL: char = '.';
const CONDITION_DAMAGED: char = '#';
const CONDITION_UNKNOWN: char = '?';

#[derive(Debug, Eq, Hash, PartialEq)]
struct Record {
    conditions: Vec<char>,
    groups: Vec<usize>,
}

impl Record {
    fn skip(&self, n: usize) -> Self {
        assert!(n <= self.conditions.len());
        Self { conditions: self.conditions[n..].to_vec(), groups: self.groups.to_vec() }
    }

    fn next(&self) -> Self {
        assert!(!self.groups.is_empty());
        let n = min(self.groups[0] + 1, self.conditions.len());
        Self { conditions: self.conditions[n..].to_vec(), groups: self.groups[1..].to_vec() }
    }

    fn replace(&self, i: usize, c: char) -> Self {
        let mut conditions = self.conditions.clone();
        conditions[i] = c;
        Self { conditions, groups: self.groups.clone() }
    }

    fn unfold(&self) -> Self {
        let mut conditions = Vec::new();
        for i in 0..5 {
            if i > 0 {
                conditions.push(CONDITION_UNKNOWN);
            }
            conditions.extend(&self.conditions);
        }
        Self { conditions, groups: self.groups.repeat(5) }
    }
}

impl Clone for Record {
    fn clone(&self) -> Self {
        Self { conditions: self.conditions.to_vec(), groups: self.groups.to_vec() }
    }
}

#[memoize]
fn count(rec: Record) -> u64 {
    if rec.conditions.is_empty() {
        if rec.groups.is_empty() { 1 } else { 0 }
    } else if rec.groups.is_empty() {
        if rec.conditions.contains(&CONDITION_DAMAGED) { 0 } else { 1 }
    } else {
        match rec.conditions[0] {
            CONDITION_OPERATIONAL => {
                count(rec.skip(1))
            },
            CONDITION_DAMAGED => {
                let group_len = rec.groups[0];
                if rec.conditions.len() >= group_len
                    && !rec.conditions[..group_len].iter().any(|&c| c == CONDITION_OPERATIONAL)
                    && (rec.conditions.len() == group_len || rec.conditions[group_len] != CONDITION_DAMAGED) {
                    count(rec.next())
                } else {
                    0
                }
            }
            CONDITION_UNKNOWN => {
                count(rec.replace(0, CONDITION_DAMAGED)) + count(rec.replace(0, CONDITION_OPERATIONAL))
            },
            c => panic!("unsupported condition <{}>", c)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    fn new_record(conditions: &str, groups: &[usize]) -> Record {
        Record { conditions: conditions.chars().collect(), groups: groups.to_vec() }
    }

    #[test]
    fn test_parse() {
        let record = parse("#.#.### 1,1,3");
        assert_eq!(record.len(), 1);
        assert_eq!(record[0].conditions, vec!['#', '.', '#', '.', '#', '#', '#']);
        assert_eq!(record[0].groups, vec![1, 1, 3]);
    }

    #[test]
    fn test_skip() {
        let record = new_record(".#.#", &[2, 1]).skip(1);
        assert_eq!(record.conditions, vec!['#', '.', '#']);
        assert_eq!(record.groups, vec![2, 1]);
    }

    #[test]
    fn test_next() {
        let record = new_record(".#.#", &[2, 1]).next();
        assert_eq!(record.conditions, vec!['#']);
        assert_eq!(record.groups, vec![1]);
    }

    #[test_case("???.###", &[1, 1, 3], 1 ; "record 1")]
    #[test_case(".??..??...?##.", &[1, 1, 3], 4 ; "record 2")]
    #[test_case("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], 1 ; "record 3")]
    #[test_case("????.#...#...", &[4, 1, 1], 1 ; "record 4")]
    #[test_case("????.######..#####.", &[1, 6, 5], 4 ; "record 5")]
    #[test_case("?###????????", &[3, 2, 1], 10 ; "record 6")]
    fn test_count(conditions: &str, groups: &[usize], expected: u64) {
        let record = new_record(conditions, groups);
        assert_eq!(count(record), expected);
    }

    #[test]
    fn test_solve_part_1() {
        let records = parse("
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ");
        assert_eq!(solve_part_1(&records), 21);
    }

    #[test]
    fn test_unfold() {
        let record = new_record(".#", &[]);
        assert_eq!(record.unfold().conditions, vec![
            '.', '#', '?',
            '.', '#', '?',
            '.', '#', '?',
            '.', '#', '?',
            '.', '#'
        ]);
    }

    #[test_case("???.###", &[1, 1, 3], 1 ; "record 1")]
    #[test_case(".??..??...?##.", &[1, 1, 3], 16384 ; "record 2")]
    #[test_case("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], 1 ; "record 3")]
    #[test_case("????.#...#...", &[4, 1, 1], 16 ; "record 4")]
    #[test_case("????.######..#####.", &[1, 6, 5], 2500 ; "record 5")]
    #[test_case("?###????????", &[3, 2, 1], 506250 ; "record 6")]
    fn test_count_unfolded(conditions: &str, groups: &[usize], expected: u64) {
        let record = new_record(conditions, groups).unfold();
        assert_eq!(count(record), expected);
    }

    #[test]
    fn test_solve_part_2() {
        let records = parse("
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ");
        assert_eq!(solve_part_2(&records), 525_152);
    }
}
