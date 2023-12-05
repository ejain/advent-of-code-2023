use std::fs::read_to_string;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/01.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += extract(line);
    }
    sum
}

fn extract(s: &str) -> u32 {
    to_number(s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>())
}

fn to_number(digits: Vec<char>) -> u32 {
    let first = digits.first().cloned();
    let last = digits.last().cloned().or(first);
    concat_digits(first, last)
}

fn concat_digits(first: Option<char>, last: Option<char>) -> u32 {
    if first.is_none() || last.is_none() {
        return 0
    }
    format!("{}{}", first.unwrap(), last.unwrap()).parse().unwrap()
}

fn solve_part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let first = find_first(line);
        let last = find_last(line);
        sum += concat_digits(first, last);
    }
    sum
}

fn find_first(input: &str) -> Option<char> {
    let re = regex!("([1-9]|one|two|three|four|five|six|seven|eight|nine)");
    if let Some(m) = re.find(input) {
        return match m.as_str() {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            digit => digit.chars().next(),
        }
    }
    None
}

fn find_last(input: &str) -> Option<char> {
    let re = regex!("([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)");
    if let Some(m) = re.find(&reverse(input)) {
        return match m.as_str() {
            "eno" => Some('1'),
            "owt" => Some('2'),
            "eerht" => Some('3'),
            "ruof" => Some('4'),
            "evif" => Some('5'),
            "xis" => Some('6'),
            "neves" => Some('7'),
            "thgie" => Some('8'),
            "enin" => Some('9'),
            digit => digit.chars().next(),
        }
    }
    None
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract() {
        assert_eq!(extract(""), 0);
        assert_eq!(extract("abc"), 0);
        assert_eq!(extract("4"), 44,);
        assert_eq!(extract("a4bc"), 44);
        assert_eq!(extract("42"), 42);
        assert_eq!(extract("124"), 14);
        assert_eq!(extract("a1bc2def4"), 14);
    }

    #[test]
    fn test_find_first() {
        assert_eq!(find_first(""), None);
        assert_eq!(find_first("7"), Some('7'));
        assert_eq!(find_first("42"), Some('4'));
        assert_eq!(find_first("abc"), None);
        assert_eq!(find_first("one"), Some('1'));
        assert_eq!(find_first("oneight"), Some('1'));
        assert_eq!(find_first("abconetwothreedef"), Some('1'));
    }

    #[test]
    fn test_find_last() {
        assert_eq!(find_last(""), None);
        assert_eq!(find_last("7"), Some('7'));
        assert_eq!(find_last("42"), Some('2'));
        assert_eq!(find_last("abc"), None);
        assert_eq!(find_last("one"), Some('1'));
        assert_eq!(find_last("oneight"), Some('8'));
        assert_eq!(find_last("abconetwothreedef"), Some('3'));
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1("
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "), 142);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2("
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "), 281);
    }
}
