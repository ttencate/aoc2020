const PRIME: u64 = 20201227;

fn loop_size(pub_key: u64, subject_number: u64) -> u64 {
    let mut power = 1;
    let mut loop_size = 0;
    while power != pub_key {
        power = (power * subject_number) % PRIME;
        loop_size += 1;
    }
    loop_size
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % PRIME;
    }
    value
}

fn part1(input: &str) -> u64 {
    let mut parts = input.lines().map(|line| line.parse::<u64>().unwrap());
    let pub_key_a = parts.next().unwrap();
    let pub_key_b = parts.next().unwrap();
    let loop_size_a = loop_size(pub_key_a, 7);
    transform(pub_key_b, loop_size_a)
}

#[test]
fn test_part1() {
    assert_eq!(part1("17807724\n5764801"), 14897079);
    assert_eq!(part1("5764801\n17807724"), 14897079);
    assert_eq!(part1(&aoc::input()), 3803729);
}

fn part2(_input: &str) -> String {
    "n/a".to_string()
}

fn main() {
    aoc::main(part1, part2);
}
