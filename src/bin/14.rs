use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

enum Instr {
    Mask { zero: u64, one: u64, x: u64 },
    Mem { addr: u64, val: u64 },
}

use Instr::*;

fn mask_bits(mask: &str, chr: u8) -> u64 {
    mask.bytes().zip((0..36).rev())
        .map(|(c, i)| if c == chr { 1 << i } else { 0 })
        .fold(0, |a, b| a | b)
}

fn parse_instr(line: &str) -> Instr {
    if let Some(caps) = MEM_RE.captures(line) {
        Mem {
            addr: caps.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            val: caps.get(2).unwrap().as_str().parse::<u64>().unwrap(),
        }
    } else if let Some(caps) = MASK_RE.captures(line) {
        let mask = caps.get(1).unwrap().as_str();
        Mask {
            zero: mask_bits(mask, b'0'),
            one: mask_bits(mask, b'1'),
            x: mask_bits(mask, b'X'),
        }
    } else {
        panic!()
    }
}

fn part1(input: &str) -> u64 {
    input.lines().map(parse_instr)
        .fold((0, 0, HashMap::new()), |(zero_mask, one_mask, mut mem), instr| {
            match instr {
                Mask { zero, one, .. } => (zero, one, mem),
                Mem { addr, val } => {
                    mem.insert(addr, (val & !zero_mask) | one_mask);
                    (zero_mask, one_mask, mem)
                }
            }
        })
        .2.values().sum()
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
        let bit = 1 << floating_mask.trailing_zeros();
        set_floating(mem, addr & !bit, floating_mask & !bit, val);
        set_floating(mem, addr | bit, floating_mask & !bit, val);
    }
}

fn part2(input: &str) -> u64 {
    input.lines().map(parse_instr)
        .fold((0, 0, HashMap::new()), |(one_mask, floating_mask, mut mem), instr| {
            match instr {
                Mask { one, x, .. } => (one, x, mem),
                Mem { addr, val } => {
                    set_floating(&mut mem, addr | one_mask, floating_mask, val);
                    (one_mask, floating_mask, mem)
                },
            }
        })
        .2.values().sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(4)), 208);
    assert_eq!(part2(&aoc::input()), 4401465949086);
}

fn main() {
    aoc::main(part1, part2);
}
