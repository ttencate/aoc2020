use regex::Regex;

fn parse(input: &str) -> impl Iterator<Item = (usize, usize, char, &str)> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    input.lines()
        .map(move |line| {
            let caps = re.captures(line).unwrap();
            let min = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let max = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let ch = caps.get(3).unwrap().as_str().chars().next().unwrap();
            let pw = caps.get(4).unwrap().as_str();
            (min, max, ch, pw)
        })
}

fn part1(input: &str) -> u64 {
    parse(input)
        .filter(|&(min, max, ch, pw)| {
            let count = pw.chars().filter(|&c| c == ch).count();
            min <= count && count <= max
        })
        .count() as u64
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 2);
    assert_eq!(part1(&aoc::input()), 614);
}

fn part2(input: &str) -> u64 {
    parse(input)
        .filter(|&(min, max, ch, pw)| {
            (pw.chars().nth(min - 1).unwrap() == ch) != (pw.chars().nth(max - 1).unwrap() == ch)
        })
        .count() as u64
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 1);
    assert_eq!(part2(&aoc::input()), 354);
}

fn main() {
    aoc::main(part1, part2);
}
