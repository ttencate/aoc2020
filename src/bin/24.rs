use std::collections::HashSet;

fn final_tile(line: &str) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    let mut bytes = line.bytes();
    while let Some(c) = bytes.next() {
        let (dx, dy) = match c {
            b'e' => (1, 0),
            b'w' => (-1, 0),
            b'n' => match bytes.next().unwrap() {
                b'e' => (1, -1),
                b'w' => (0, -1),
                _ => panic!(),
            },
            b's' => match bytes.next().unwrap() {
                b'e' => (0, 1),
                b'w' => (-1, 1),
                _ => panic!(),
            },
            _ => panic!(),
        };
        x += dx;
        y += dy;
    }
    (x, y)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| final_tile(line))
        .fold(HashSet::new(), |mut black, tile| {
            if black.contains(&tile) {
                black.remove(&tile);
            } else {
                black.insert(tile);
            }
            black
        })
        .len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 10);
    assert_eq!(part1(&aoc::input()), 322);
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
