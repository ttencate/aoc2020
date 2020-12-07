use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Color(String);

struct Rule {
    outer: Color,
    inner: Vec<(u64, Color)>,
}

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(?P<outer>.*?) bags contain (?P<inner>.*)\.$").unwrap();
    static ref INNER_RE: Regex = Regex::new(r"^(?P<count>\d+) (?P<color>.*?) bag[s]?").unwrap();
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RULE_RE.captures(s).unwrap();
        let outer = Color(caps.name("outer").unwrap().as_str().to_string());
        let inner_str = caps.name("inner").unwrap().as_str();
        let inner = if inner_str == "no other bags" {
            vec![]
        } else {
            inner_str
                .split(", ")
                .map(|s| {
                    let caps = INNER_RE.captures(s).unwrap();
                    let count = caps.name("count").unwrap().as_str().parse::<u64>().unwrap();
                    let color = Color(caps.name("color").unwrap().as_str().to_string());
                    (count, color)
                })
                .collect()
        };
        Ok(Rule {
            outer,
            inner,
        })
    }
}

fn part1(input: &str) -> usize {
    let rules = input
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();

    let mut inner_to_outer = HashMap::<&Color, Vec<&Color>>::new();
    for rule in &rules {
        for (_, inner) in &rule.inner {
            inner_to_outer.entry(&inner).or_default().push(&rule.outer);
            inner_to_outer.entry(&rule.outer).or_default();
        }
    }

    let innermost = Color("shiny gold".to_string());
    let mut outermost = HashSet::<&Color>::new();
    let mut stack = vec![&innermost];
    while let Some(curr) = stack.pop() {
        if !outermost.contains(curr) {
            outermost.insert(curr);
            for outer in inner_to_outer.get(curr).unwrap() {
                stack.push(outer);
            }
        }
    }

    outermost.len() - 1 // "shiny gold" is not counted as containing itself.
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 4);
    assert_eq!(part1(&aoc::input()), 142);
}

fn count_contained_bags(outer: &Color, outer_to_rule: &HashMap<&Color, &Rule>) -> u64 {
    let rule = outer_to_rule.get(outer).unwrap();
    rule.inner
        .iter()
        .map(|(count, color)| {
            count * (1 + count_contained_bags(color, outer_to_rule))
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let rules = input
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();
    let outer_to_rule = rules
        .iter()
        .map(|rule| (&rule.outer, rule))
        .collect::<HashMap<_, _>>();
    count_contained_bags(&Color("shiny gold".to_string()), &outer_to_rule)
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 32);
    assert_eq!(part2(&aoc::example(1)), 126);
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
