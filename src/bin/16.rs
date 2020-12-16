use bit_set::BitSet;
use std::collections::HashMap;

fn parse_rule(line: &str) -> (String, BitSet) {
    let mut parts = line.split(": ");

    let name = parts.next().unwrap().to_string();

    let valid = parts
        .next()
        .unwrap()
        .split(" or ")
        .map(|range| {
            let mut parts = range.split("-");
            let lo = parts.next().unwrap().parse::<usize>().unwrap();
            let hi = parts.next().unwrap().parse::<usize>().unwrap() + 1;
            assert!(parts.next().is_none());
            (lo..hi).collect::<BitSet>()
        })
        .fold(BitSet::with_capacity(1024), |a, b| a.union(&b).collect());

    assert!(parts.next().is_none());

    (name, valid)
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(",").map(|p| p.parse::<usize>().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut valid_for_any_field = BitSet::with_capacity(1024);
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (_name, valid) = parse_rule(line);
        valid_for_any_field.union_with(&valid);
    }

    assert_eq!(lines.next().unwrap(), "your ticket:");
    let _my_ticket = parse_ticket(lines.next().unwrap());
    assert!(lines.next().unwrap().is_empty());

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let mut error_rate = 0;
    while let Some(line) = lines.next() {
        error_rate += parse_ticket(line)
            .into_iter()
            .filter(|&field| !valid_for_any_field.contains(field))
            .sum::<usize>();
    }
    error_rate
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(1)), 71);
    assert_eq!(part1(&aoc::input()), 30869);
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();

    let mut fields = Vec::<(String, BitSet)>::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        fields.push(parse_rule(line));
    }
    let valid_for_any_field = fields
        .iter()
        .map(|(_name, valid)| valid)
        .fold(BitSet::with_capacity(1024), |a, b| a.union(b).collect::<BitSet>());

    assert_eq!(lines.next().unwrap(), "your ticket:");
    let my_ticket = parse_ticket(lines.next().unwrap());
    assert!(lines.next().unwrap().is_empty());

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let n = fields.len();
    let mut candidates = (0..n)
        .map(|_| (0..n).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    while let Some(line) = lines.next() {
        let ticket = parse_ticket(line);
        if !ticket.iter().all(|&field| valid_for_any_field.contains(field)) {
            continue;
        }
        for (i, &f) in ticket.iter().enumerate() {
            let cand = &mut candidates[i];
            let mut j = 0;
            while j < cand.len() {
                if !fields[cand[j]].1.contains(f) {
                    cand.swap_remove(j);
                } else {
                    j += 1;
                }
            }
        }
    }

    let mut ticket_indices_for_field = HashMap::<String, usize>::new();
    while let Some(ticket_index) = candidates.iter().position(|c| c.len() == 1) {
        let field_index = *candidates[ticket_index].iter().next().unwrap();
        let field_name = &fields[field_index].0;
        assert!(!ticket_indices_for_field.contains_key(field_name));
        ticket_indices_for_field.insert(field_name.to_string(), ticket_index);
        for c in candidates.iter_mut() {
            if let Some(ci) = c.iter().position(|&fi| fi == field_index) {
                c.swap_remove(ci);
            }
        }
    }
    assert!(ticket_indices_for_field.len() == n);
    
    fields
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(name, _)| my_ticket[*ticket_indices_for_field.get(name).unwrap()])
        .product()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::input()), 4381476149273);
}

fn main() {
    aoc::main(part1, part2);
}
