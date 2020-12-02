use std::collections::HashSet;

fn parse(input: &str) -> HashSet<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<_>>()
}

fn part1(input: &str) -> u64 {
    let numbers = parse(input);
    for n in &numbers {
        let m = 2020 - n;
        if numbers.contains(&m) {
            return n * m;
        }
    }
    panic!("not found");
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 514579);
    assert_eq!(part1(&aoc::input()), 1005459);
}

fn part2(input: &str) -> u64 {
    let numbers = parse(input);
    for n in &numbers {
        for m in &numbers {
            if m + n <= 2020 {
                let k = 2020 - m - n;
                if numbers.contains(&k) {
                    return n * m * k;
                }
            }
        }
    }
    panic!("not found");
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 241861950);
    assert_eq!(part2(&aoc::input()), 92643264);
}

fn main() {
    aoc::main(part1, part2);
}
