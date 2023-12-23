use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/19.txt").unwrap();
    let (system, parts) = parse(&input);
    println!("Part 1: {}", solve_part_1(&system, &parts));
    println!("Part 2: {}", solve_part_2(&system, &parts));
}

fn parse(input: &str) -> (System, Vec<Part>) {
    let mut system = System { workflows: HashMap::new() };
    let mut parts = Vec::new();
    for line in input.lines().map(str::trim).filter(|&line| !line.is_empty()) {
        if line.starts_with('{') {
            parts.push(Part::parse(line));
        } else {
            let workflow = Workflow::parse(line);
            let label = workflow.label.to_string();
            system.workflows.insert(label, workflow);
        }
    }
    (system, parts)
}

fn solve_part_1(system: &System, parts: &[Part]) -> u32 {
    parts.iter()
        .filter(|&part| system.accept(part))
        .map(|part| part.sum())
        .sum()
}

fn solve_part_2(system: &System, _parts: &[Part]) -> u64 {
    system.analyze().iter()
        .map(|range| range.count())
        .sum()
}

struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    fn accept(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        loop {
            match workflow.accept(part) {
                Result::Accept => return true,
                Result::Reject => return false,
                Result::Continue(label) => workflow = self.workflows.get(&label).unwrap(),
            }
        }
    }

    fn analyze(&self) -> Vec<PartRange> {
        self._analyze("in", PartRange::new())
    }

    fn _analyze(&self, workflow_label: &str, base_range: PartRange) -> Vec<PartRange> {
        let mut ranges = Vec::new();
        let workflow = self.workflows.get(workflow_label).unwrap();
        for (range, result) in workflow.analyze(base_range) {
            match result {
                Result::Accept => ranges.push(range),
                Result::Reject => (),
                Result::Continue(label) => ranges.extend(self._analyze(&label, range)),
            }
        }
        ranges
    }
}

struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let re = regex!(r"([a-z]+)\{(.+)\}");
        if let Some(m) = re.captures(line) {
            let label = m.get(1).unwrap().as_str().to_string();
            let rules = m.get(2).unwrap().as_str().split(',')
                .map(Rule::parse)
                .collect();
            Self { label, rules }
        } else {
            panic!("can't parse workflow <{}>", line);
        }
    }

    fn accept(&self, part: &Part) -> Result {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.result.clone()
            }
        }
        panic!("no fallback in rule");
    }

    fn analyze(&self, range: PartRange) -> Vec<(PartRange, Result)> {
        let mut results = Vec::new();
        let mut range = range;
        for rule in &self.rules {
            let mut r = rule.condition.analyze();
            r.add(&range);
            results.push((r, rule.result.clone()));
            range.add(&rule.condition.invert().analyze());
        }
        results
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    condition: Condition,
    result: Result,
}

impl Rule {
    fn parse(s: &str) -> Self {
        let tokens = s.split(':').collect::<Vec<&str>>();
        match tokens.len() {
            1 => Self { condition: Condition::Any, result: Result::parse(s) },
            2 => Self { condition: Condition::parse(tokens[0]), result: Result::parse(tokens[1]) },
            _ => panic!("can't parse rule <{}>", s),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        self.condition.eval(part)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Condition {
    GreaterThan(String, u32),
    LessThan(String, u32),
    Any,
}

impl Condition {
    fn parse(s: &str) -> Self {
        let re = regex!(r"([a-z]+)(<|>)(\d+)");
        if let Some(m) = re.captures(s) {
            let property = m.get(1).unwrap().as_str().to_string();
            let operator = m.get(2).unwrap().as_str();
            let value = m.get(3).unwrap().as_str().parse().unwrap();
            match operator {
                "<" => Self::LessThan(property, value),
                ">" => Self::GreaterThan(property, value),
                _ => panic!("unsupported operator <{}>", operator),
            }
        } else {
            Self::Any
        }
    }

    fn eval(&self, part: &Part) -> bool {
        match self {
            Self::GreaterThan(label, value) => part.properties[label] > *value,
            Self::LessThan(label, value) => part.properties[label] < *value,
            Self::Any => true,
        }
    }

    fn invert(&self) -> Condition {
        match self {
            Self::GreaterThan(label, value) => Self::LessThan(label.to_string(), value + 1),
            Self::LessThan(label, value) => Self::GreaterThan(label.to_string(), value - 1),
            Self::Any => Self::Any,
        }
    }

    fn analyze(&self) -> PartRange {
        let mut range = PartRange::new();
        match self {
            Self::GreaterThan(label, value) => {
                range.increase_start(label, *value + 1);
            },
            Self::LessThan(label, value) => {
                range.reduce_end(label, *value);
            },
            Self::Any => (),
        }
        range
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Result {
    Accept,
    Reject,
    Continue(String),
}

impl Result {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Result::Accept,
            "R" => Result::Reject,
            label => Result::Continue(label.to_string()),
        }
    }
}

struct Part {
    properties: HashMap<String, u32>,
}

impl Part {
    fn parse(line: &str) -> Self {
        let mut part = Self { properties: HashMap::new() };
        let re = regex!(r"([a-z])=(\d+)");
        for (_, [property, value]) in re.captures_iter(line).map(|c| c.extract()) {
            part.properties.insert(property.to_string(), value.parse().unwrap());
        }
        part
    }

    fn sum(&self) -> u32 {
        self.properties.values().sum()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct PartRange {
    properties: HashMap<String, Range<u32>>,
}

impl PartRange {
    fn new() -> Self {
        let mut properties = HashMap::new();
        for label in ["x", "m", "a", "s"] {
            properties.insert(label.to_string(), 1..4001);
        }
        Self { properties }
    }

    fn reduce_end(&mut self, label: &str, value: u32) {
        if let Some(range) = self.properties.get(label) {
            if range.end > value {
                self.properties.insert(label.to_string(), range.start..value);
            }
        } else {
            self.properties.insert(label.to_string(), 1..value);
        }
    }

    fn increase_start(&mut self, label: &str, value: u32) {
        if let Some(range) = self.properties.get(label) {
            if range.start < value {
                self.properties.insert(label.to_string(), value..range.end);
            }
        } else {
            self.properties.insert(label.to_string(), value..4001);
        }
    }

    fn add(&mut self, other: &PartRange) {
        for (label, range) in &other.properties {
            self.increase_start(label, range.start);
            self.reduce_end(label, range.end);
        }
    }

    fn count(&self) -> u64 {
        self.properties.values()
            .map(|range| range.end as u64 - range.start as u64)
            .reduce(|left, right| left * right)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (System, Vec<Part>) {
        parse("
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        ")
    }

    #[test]
    fn test_parse() {
        let (system, parts) = setup();
        assert_eq!(system.workflows.keys().len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn test_accept() {
        let (system, _) = setup();
        let part = Part { properties: HashMap::from([
            ("x".to_string(), 787),
            ("m".to_string(), 2655),
            ("a".to_string(), 1222),
            ("s".to_string(), 2876)
        ]) };
        assert!(system.accept(&part));
    }

    #[test]
    fn test_reject() {
        let (system, _) = setup();
        let part = Part { properties: HashMap::from([
            ("x".to_string(), 1679),
            ("m".to_string(), 44),
            ("a".to_string(), 2067),
            ("s".to_string(), 496)
        ]) };
        assert!(!system.accept(&part));
    }

    #[test]
    fn test_solve_part_1() {
        let (system, parts) = setup();
        assert_eq!(solve_part_1(&system, &parts), 19114);
    }

    #[test]
    fn test_part_range() {
        let mut range = PartRange::new();
        assert_eq!(range.count(), 4000 * 4000 * 4000 * 4000);
        range.reduce_end("a", 2001);
        assert_eq!(range.count(), 2000 * 4000 * 4000 * 4000);
        range.increase_start("a", 1001);
        assert_eq!(range.count(), 1000 * 4000 * 4000 * 4000);
    }

    #[test]
    fn test_solve_part_2() {
        let (system, parts) = setup();
        assert_eq!(solve_part_2(&system, &parts), 167_409_079_868_000);
    }
}
