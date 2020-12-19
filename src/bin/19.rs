use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

enum Rule {
    Char(char),
    Alternatives(Vec<Vec<usize>>),
}

fn build_re(rules: &HashMap<usize, Rule>, cache: &mut HashMap<usize, String>, name: usize) -> String {
    if !cache.contains_key(&name) {
        let re = match rules.get(&name).unwrap() {
            Rule::Char(c) => c.to_string(),
            Rule::Alternatives(alts) => {
                let re = alts.iter()
                    .map(|alt| {
                        alt.iter()
                            .map(|&n| build_re(rules, cache, n))
                            .join("")
                    })
                .join("|");
                format!("({})", re)
            }
        };
        cache.insert(name, re);
    }
    cache.get(&name).unwrap().to_string()
}

fn build_full_re(rules: &HashMap<usize, Rule>) -> Regex {
    let mut cache = HashMap::new();
    let re = format!("^{}$", build_re(rules, &mut cache, 0));
    Regex::new(&re).unwrap()
}

fn parse_rules<'a>(lines: &mut impl Iterator<Item = &'a str>) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().parse::<usize>().unwrap();
        let expansion = parts.next().unwrap();
        let rule = if expansion.starts_with("\"") {
            Rule::Char(expansion.chars().nth(1).unwrap())
        } else {
            let alts = expansion
                .split(" | ")
                .map(|alt| {
                    alt.split(" ").map(|n| n.parse::<usize>().unwrap()).collect()
                })
                .collect();
            Rule::Alternatives(alts)
        };
        rules.insert(name, rule);
    }
    rules
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let re = build_full_re(&parse_rules(&mut lines));
    lines.filter(|line| re.is_match(line)).count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(2)), 2);
    assert_eq!(part1(&aoc::input()), 230);
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

#[test]
fn test_part2() {
    // assert_eq!(part2(&aoc::example(0)), );
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
