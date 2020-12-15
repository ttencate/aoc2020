use std::collections::HashMap;

fn play(input: &str, num_turns: u64) -> u64 {
    let mut list = input
        .trim()
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .into_iter();
    let mut mem = HashMap::<u64, u64>::new();
    let mut last_spoken = 0;
    for turn in 1.. {
        let spoken = if let Some(from_list) = list.next() {
            from_list
        } else {
            match mem.get(&last_spoken) {
                None => 0,
                Some(m) => turn - 1 - m,
            }
        };
        if turn == num_turns {
            return spoken;
        }
        if turn > 1 {
            mem.insert(last_spoken, turn - 1);
        }
        last_spoken = spoken;
    }
    panic!()
}

fn part1(input: &str) -> u64 {
    play(input, 2020)
}

#[test]
fn test_part1() {
    assert_eq!(part1("0,3,6"), 436);
    assert_eq!(part1("1,3,2"), 1);
    assert_eq!(part1("2,1,3"), 10);
    assert_eq!(part1("1,2,3"), 27);
    assert_eq!(part1("2,3,1"), 78);
    assert_eq!(part1("3,2,1"), 438);
    assert_eq!(part1("3,1,2"), 1836);
    assert_eq!(part1(&aoc::input()), 257);
}

fn part2(input: &str) -> u64 {
    play(input, 30000000)
}

#[test]
fn test_part2() {
    assert_eq!(part2("0,3,6"), 175594);
    assert_eq!(part2("1,3,2"), 2578);
    assert_eq!(part2("2,1,3"), 3544142);
    assert_eq!(part2("1,2,3"), 261214);
    assert_eq!(part2("2,3,1"), 6895259);
    assert_eq!(part2("3,2,1"), 18);
    assert_eq!(part2("3,1,2"), 362);
    assert_eq!(part2(&aoc::input()), 8546398);
}

fn main() {
    aoc::main(part1, part2);
}
