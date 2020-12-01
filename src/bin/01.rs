use std::collections::HashSet;

fn part1(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<_>>();
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
    assert_eq!(part1("1721
979
366
299
675
1456"), 514579);
    aoc::test(part1, 1005459);
}

fn part2(input: &str) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<_>>();
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
    assert_eq!(part2("1721
979
366
299
675
1456"), 241861950);
    aoc::test(part2, 92643264);
}

fn main() {
    aoc::main(part1, part2);
}
