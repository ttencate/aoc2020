use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

fn mask_bits(mask: &str, chr: u8) -> u64 {
    mask.bytes().zip((0..36).rev())
        .map(|(c, i)| if c == chr { 1 << i } else { 0 })
        .fold(0, |a, b| a | b)
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut zero_mask = 0;
    let mut one_mask = 0;
    let mut mem = HashMap::new();
    while let Some(line) = lines.next() {
        if let Some(caps) = MEM_RE.captures(line) {
            let addr = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            mem.insert(addr, (val & !zero_mask) | one_mask);
        } else if let Some(caps) = MASK_RE.captures(line) {
            let mask = caps.get(1).unwrap().as_str().to_string();
            zero_mask = mask_bits(&mask, b'0');
            one_mask = mask_bits(&mask, b'1');
        } else {
            panic!();
        }
    }
    mem.values().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 165);
    assert_eq!(part1(&aoc::input()), 17765746710228);
}

fn set_floating(mem: &mut HashMap<u64, u64>, addr: u64, floating_mask: u64, val: u64) {
    if floating_mask == 0 {
        mem.insert(addr, val);
    } else {
        let bit = 1 << floating_mask.trailing_zeros() as u64;
        set_floating(mem, addr & !bit, floating_mask & !bit, val);
        set_floating(mem, addr | bit, floating_mask & !bit, val);
    }
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut one_mask = 0;
    let mut floating_mask = 0;
    let mut mem = HashMap::new();
    while let Some(line) = lines.next() {
        if let Some(caps) = MEM_RE.captures(line) {
            let addr = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            set_floating(&mut mem, addr | one_mask, floating_mask, val);
        } else if let Some(caps) = MASK_RE.captures(line) {
            let mask = caps.get(1).unwrap().as_str().to_string();
            one_mask = mask_bits(&mask, b'1');
            floating_mask = mask_bits(&mask, b'X');
        } else {
            panic!();
        }
    }
    mem.values().sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(4)), 208);
    assert_eq!(part2(&aoc::input()), 4401465949086);
}

fn main() {
    aoc::main(part1, part2);
}
