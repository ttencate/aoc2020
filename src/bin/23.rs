use itertools::Itertools;

fn part1(input: &str) -> String {
    let mut cups = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8 - 1)
        .collect::<Vec<u8>>();
    for _ in 0..100 {
        let curr_label = cups[0];
        let a = cups.remove(1);
        let b = cups.remove(1);
        let c = cups.remove(1);
        let mut dest_label = (curr_label + 9 - 1).rem_euclid(9);
        while !cups.contains(&dest_label) {
            dest_label = (dest_label + 9 - 1).rem_euclid(9);
        }
        let dest_idx = cups.iter().position(|&c| c == dest_label).unwrap() + 1;
        cups.insert(dest_idx, c);
        cups.insert(dest_idx, b);
        cups.insert(dest_idx, a);
        let curr = cups.remove(0);
        cups.push(curr);
    }
    while cups[0] != 0 {
        let head = cups.remove(0);
        cups.push(head);
    }
    cups.iter().skip(1).map(|cup| (cup + 1).to_string()).join("")
}

#[test]
fn test_part1() {
    assert_eq!(part1("389125467"), "67384529");
    assert_eq!(part1(&aoc::input()), "46978532");
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
