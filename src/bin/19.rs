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
                format!("(?:{})", re)
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

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);
    let mut cache = HashMap::new();
    let re42_str = build_re(&rules, &mut cache, 42);
    let re31_str = build_re(&rules, &mut cache, 31);
    let re42 = Regex::new(&re42_str).unwrap();
    let re31 = Regex::new(&re31_str).unwrap();
    let re_fst = Regex::new(&format!("^(?:{})+$", re42)).unwrap();
    let re_snd = Regex::new(&format!("^(?:{})+$", re31)).unwrap();
    lines
        .filter(|line| {
            for i in 0..line.len() {
                let fst = &line[0..i];
                let snd = &line[i..];
                if re_fst.is_match(fst) && re_snd.is_match(snd) {
                    let re42_cnt = re42.find_iter(fst).count();
                    let re31_cnt = re31.find_iter(snd).count();
                    if re42_cnt > re31_cnt {
                        return true;
                    }
                }
            }
            false
        })
        .count()
}

#[test]
fn test_part2() {
    assert_eq!(part1(&aoc::example(4)), 3);
    assert_eq!(part2(&aoc::example(4)), 12);
    assert_eq!(part2(&aoc::input()), 341);
}

fn main() {
    aoc::main(part1, part2);
}
