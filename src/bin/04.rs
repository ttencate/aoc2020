#![feature(str_split_once)]
#![feature(try_trait)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn parse(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport_lines| {
            passport_lines
                .split_whitespace()
                .map(|field| {
                    let (key, value) = field.split_once(':').unwrap();
                    (key.to_string(), value.to_string())
                })
                .collect()
        })
        .collect()
}

fn is_complete(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&key| passport.contains_key(key))
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(is_complete).count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 2);
    assert_eq!(part1(&aoc::input()), 210);
}

struct ValidationError;

type ValidationResult = Result<(), ValidationError>;

trait IntoValidationError {}
impl IntoValidationError for std::option::NoneError {}
impl IntoValidationError for std::num::ParseIntError {}

impl<T: IntoValidationError> From<T> for ValidationError {
    fn from(_: T) -> Self {
        ValidationError
    }
}

fn check(ok: bool) -> ValidationResult {
    match ok {
        true => Ok(()),
        false => Err(ValidationError),
    }
}

trait Between<T> {
    fn between(&self, lower: T, upper: T) -> ValidationResult;
}

impl<T: Ord> Between<T> for T {
    fn between(&self, lower: T, upper: T) -> ValidationResult {
        check(&lower <= self && self <= &upper)
    }
}

trait Matches {
    fn is_match(&self, re: &Regex) -> ValidationResult;
}

impl Matches for str {
    fn is_match(&self, re: &Regex) -> ValidationResult {
        check(re.is_match(self))
    }
}

enum Height {
    Cm(u64),
    In(u64),
}

impl FromStr for Height {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, ValidationError> {
        let caps = HGT_RE.captures(s)?;
        let val = caps.get(1)?.as_str().parse::<u64>()?;
        let unit = caps.get(2)?.as_str();
        Ok(match unit {
            "cm" => Height::Cm(val),
            "in" => Height::In(val),
            _ => panic!()
        })
    }
}

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn validate(passport: &HashMap<String, String>) -> ValidationResult {
    passport.get("byr")?.parse::<u64>()?.between(1920, 2002)?;
    passport.get("iyr")?.parse::<u64>()?.between(2010, 2020)?;
    passport.get("eyr")?.parse::<u64>()?.between(2020, 2030)?;
    match passport.get("hgt")?.parse::<Height>()? {
        Height::Cm(val) => val.between(150, 193)?,
        Height::In(val) => val.between(59, 76)?,
    }
    passport.get("hcl")?.is_match(&HCL_RE)?;
    passport.get("ecl")?.is_match(&ECL_RE)?;
    passport.get("pid")?.is_match(&PID_RE)?;
    Ok(())
}

fn part2(input: &str) -> usize {
    parse(input).iter().map(validate).filter(Result::is_ok).count()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(2)), 0);
    assert_eq!(part2(&aoc::example(3)), 4);
    assert_eq!(part2(&aoc::input()), 131);
}

fn main() {
    aoc::main(part1, part2);
}
