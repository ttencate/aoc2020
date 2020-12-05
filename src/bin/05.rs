fn seat_id(pass: &str) -> u64 {
    let binary = pass
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");
    u64::from_str_radix(&binary, 2).unwrap()
}

fn part1(input: &str) -> u64 {
    input.lines().map(seat_id).max().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1("FBFBBFFRLR"), 357);
    assert_eq!(part1("BFFFBBFRRR"), 567);
    assert_eq!(part1("FFFBBBFRRR"), 119);
    assert_eq!(part1("BBFFBBFRLL"), 820);
    assert_eq!(part1(&aoc::input()), 913);
}

fn part2(input: &str) -> u64 {
    let mut seat_ids = input.lines().map(seat_id).collect::<Vec<_>>();
    seat_ids.sort();
    let mut gaps = seat_ids
        .iter()
        .zip(&seat_ids[1..])
        .filter_map(|(a, b)| {
            if b - a == 2 { Some(b - 1) } else { None }
        })
        .collect::<Vec<_>>()
        .into_iter();
    let my_seat = gaps.next().unwrap();
    assert!(gaps.next().is_none());
    my_seat
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::input()), 717);
}

fn main() {
    aoc::main(part1, part2);
}
