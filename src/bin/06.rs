#![feature(iterator_fold_self)]

use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| 'a' <= c && c <= 'z')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 6);
    assert_eq!(part1(&aoc::example(1)), 11);
    assert_eq!(part1(&aoc::input()), 6596);
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .trim_end()
                .split("\n")
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold_first(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 3);
    assert_eq!(part2(&aoc::example(1)), 6);
    assert_eq!(part2(&aoc::input()), 3219);
}

fn main() {
    aoc::main(part1, part2);
}
