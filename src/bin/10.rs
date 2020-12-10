fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &str) -> u64 {
    let mut adapters = parse(input);
    adapters.push(0);
    adapters.sort();
    adapters.push(*adapters.last().unwrap() + 3);
    let diffs: Vec<u64> = adapters.iter().zip(&adapters[1..]).map(|(a, b)| b - a).collect();
    diffs.iter().filter(|&&d| d == 1).count() as u64 * diffs.iter().filter(|&&d| d == 3).count() as u64
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 7 * 5);
    assert_eq!(part1(&aoc::example(1)), 22 * 10);
    assert_eq!(part1(&aoc::input()), 2030);
}

fn part2(input: &str) -> u64 {
    let mut adapters = parse(input);
    adapters.sort();
    let mut combinations = vec![0; *adapters.last().unwrap() as usize + 4];
    combinations[3] = 1;
    for a in adapters {
        let idx = (a + 3) as usize;
        combinations[idx] = combinations[idx - 1] + combinations[idx - 2] + combinations[idx - 3];
    }
    *combinations.last().unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 8);
    assert_eq!(part2(&aoc::example(1)), 19208);
    assert_eq!(part2(&aoc::input()), 42313823813632);
}

fn main() {
    aoc::main(part1, part2);
}
