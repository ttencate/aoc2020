use multiset::HashMultiSet;

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn first_non_sum(numbers: &Vec<u64>, k: usize) -> u64 {
    let mut set = HashMultiSet::new();
    for i in 0..k {
        set.insert(numbers[i]);
    }
    for i in k..numbers.len() {
        let n = numbers[i];
        let is_sum = ((i - k)..i)
            .map(|ia| numbers[ia])
            .filter(|&a| a <= n && a + a != n && set.contains(&(n - a)))
            .next()
            .is_some();
        if !is_sum {
            return n;
        }
        set.remove(&numbers[i - k]);
        set.insert(n);
    }
    panic!();
}

fn part1(input: &str) -> u64 {
    first_non_sum(&parse(input), 25)
}

#[test]
fn test_part1() {
    assert_eq!(first_non_sum(&parse(&aoc::example(0)), 5), 127);
    assert_eq!(part1(&aoc::input()), 31161678);
}

fn weakness(input: &str, k: usize) -> u64 {
    let numbers = parse(input);
    let target = first_non_sum(&numbers, k);
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    while start < numbers.len() {
        if sum < target {
            sum += numbers[end];
            end += 1;
        } else if sum > target {
            sum -= numbers[start];
            start += 1;
        } else {
            return numbers[start..end].iter().min().unwrap() + numbers[start..end].iter().max().unwrap();
        }
    }
    panic!();
}

fn part2(input: &str) -> u64 {
    weakness(input, 25)
}

#[test]
fn test_part2() {
    assert_eq!(weakness(&aoc::example(0), 5), 62);
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
