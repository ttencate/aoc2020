fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<i64>().unwrap();
    let (wait, id) = lines.next().unwrap().split(',')
        .filter_map(|id| id.parse::<i64>().ok())
        .map(|id| ((-t).rem_euclid(id), id))
        .min().unwrap();
    id * wait
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 295);
    assert_eq!(part1(&aoc::input()), 3385);
}

/// Extended Euclidean algorithm. Returns a triple (r, s, t) such that:
///
///     gcd(a, b) = r = a*s + b*t
///
fn extended_euclid(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let quotient = old_r.div_euclid(r);
        let new_r = old_r - quotient * r; old_r = r; r = new_r;
        let new_s = old_s - quotient * s; old_s = s; s = new_s;
        let new_t = old_t - quotient * t; old_t = t; t = new_t;
    }
    (old_r, old_s, old_t)
}

#[test]
fn test_extended_euclid() {
    assert_eq!(extended_euclid(1, 1), (1, 0, 1));
    assert_eq!(extended_euclid(3, 4), (1, -1, 1));
    assert_eq!(extended_euclid(4, 3), (1, 1, -1));
    assert_eq!(extended_euclid(12, 3), (3, 0, 1));
    assert_eq!(extended_euclid(3, 12), (3, 1, 0));
    assert_eq!(extended_euclid(12, 6), (6, 0, 1));
    assert_eq!(extended_euclid(6, 12), (6, 1, 0));
    assert_eq!(extended_euclid(12, 9), (3, 1, -1));
    assert_eq!(extended_euclid(9, 12), (3, -1, 1));
}

fn part2(input: &str) -> i128 {
    input
        .lines().nth(1).unwrap().split(',')
        .zip(0i128..)
        .filter_map(|(id, i)| id.parse::<i128>().ok().map(|n| ((-i).rem_euclid(n), n)))
        .fold((0, 1), |(x, n), (ai, ni)| {
            let (_, m, mi) = extended_euclid(n, ni);
            ((x * mi * ni + ai * m * n).rem_euclid(n * ni), n * ni)
        })
        .0
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 1068781);
    assert_eq!(part2("0\n17,x,13,19"), 3417);
    assert_eq!(part2("0\n67,7,59,61"), 754018);
    assert_eq!(part2("0\n67,x,7,59,61"), 779210);
    assert_eq!(part2("0\n67,7,x,59,61"), 1261476);
    assert_eq!(part2("0\n1789,37,47,1889"), 1202161486);
    assert_eq!(part2(&aoc::input()), 600689120448303);
}

fn main() {
    aoc::main(part1, part2);
}
