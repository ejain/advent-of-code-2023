use std::collections::HashMap;
use std::fs::read_to_string;
use regex_macro::regex;

fn main() {
    let input = read_to_string("data/19.txt").unwrap();
    let (system, parts) = parse(&input);
    println!("Part 1: {}", solve_part_1(&system, &parts));
    
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

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_solve_part_1() {
        let (system, parts) = parse("
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
        ");
        assert_eq!(system.workflows.keys().len(), 11);
        assert_eq!(parts.len(), 5);
        assert_eq!(solve_part_1(&system, &parts), 19114);
    }
}
