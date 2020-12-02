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
    assert_eq!(part1("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), 2);
    aoc::test(part1, 614);
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
    assert_eq!(part2("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), 1);
    aoc::test(part2, 354);
}

fn main() {
    aoc::main(part1, part2);
}
