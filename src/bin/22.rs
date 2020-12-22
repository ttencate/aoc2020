use std::collections::VecDeque;

fn part1(input: &str) -> u64 {
    let mut decks = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.parse::<u64>().unwrap())
                .collect::<VecDeque<u64>>()
        })
        .collect::<Vec<VecDeque<u64>>>();

    while !decks[0].is_empty() && !decks[1].is_empty() {
        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();
        if card0 > card1 {
            decks[0].push_back(card0);
            decks[0].push_back(card1);
        } else {
            decks[1].push_back(card1);
            decks[1].push_back(card0);
        }
    }

    decks.iter()
        .flatten()
        .rev()
        .zip(1..)
        .map(|(c, i)| c * i)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 306);
    assert_eq!(part1(&aoc::input()), 33098);
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
