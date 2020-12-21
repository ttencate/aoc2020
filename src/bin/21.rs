use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
}

fn parse<'a>(input: &'a str) -> Vec<Food<'a>> {
    input
        .lines()
        .map(|line| {
            let caps = LINE_RE.captures(line).unwrap();
            let ingredients = caps.get(1).unwrap().as_str().split(" ").collect::<Vec<&str>>();
            let allergens = caps.get(2).unwrap().as_str().split(", ").collect::<Vec<&str>>();
            Food { ingredients, allergens }
        })
        .collect()
}

fn candidates<'a>(foods: &Vec<Food<'a>>) -> HashMap<&'a str, Vec<&'a str>> {
    let all_ingredients = foods.iter()
        .flat_map(|food| food.ingredients.iter().copied())
        .collect::<Vec<&str>>();
    let all_allergens = foods.iter()
        .flat_map(|food| food.allergens.iter().copied())
        .collect::<Vec<&str>>();

    let mut candidates = all_allergens.iter()
        .map(|&allergen| (allergen, all_ingredients.clone()))
        .collect::<HashMap<&str, Vec<&str>>>();
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            candidates.get_mut(allergen).unwrap()
                .retain(|ingredient| food.ingredients.contains(ingredient));
        }
    }

    candidates
}

fn part1(input: &str) -> usize {
    let foods = parse(input);

    let candidates = candidates(&foods);

    let all_ingredients = foods.iter()
        .flat_map(|food| food.ingredients.iter().copied())
        .collect::<HashSet<&str>>();

    let mut safe_ingredients = all_ingredients.iter().copied().collect::<HashSet<&str>>();
    for (_, ingredients) in candidates {
        for ingredient in ingredients {
            safe_ingredients.remove(ingredient);
        }
    }

    foods.iter()
        .map(|food| {
            food.ingredients.iter()
                .filter(|&ingredient| safe_ingredients.contains(ingredient))
                .count()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 5);
    assert_eq!(part1(&aoc::input()), 2389);
}

fn solve<'a, 'b>(candidates: &'b mut Vec<(&'a str, HashSet<&'a str>)>, solution: &'b mut Vec<(&'a str, &'a str)>) -> bool {
    let i = solution.len();
    if i == candidates.len() {
        return true;
    }

    if candidates[i].1.is_empty() {
        return false;
    }

    for ingredient in candidates[i].1.iter().copied().collect::<Vec<&str>>() {
        let mut removed_from = Vec::<usize>::new();
        for j in (i + 1)..candidates.len() {
            if candidates[j].1.remove(ingredient) {
                removed_from.push(j);
            }
        }
        solution.push((candidates[i].0, ingredient));
        if solve(candidates, solution) {
            return true;
        }
        solution.pop();
        for j in removed_from {
            candidates[j].1.insert(ingredient);
        }
    }

    false
}

fn part2(input: &str) -> String {
    let foods = parse(input);

    let mut candidates = candidates(&foods).iter()
        .map(|(&allergen, ingredients)| (allergen, ingredients.iter().copied().collect::<HashSet<&str>>()))
        .collect::<Vec<(&str, HashSet<&str>)>>();
    candidates.sort_by_key(|(_, ingredients)| ingredients.len());

    let mut solution = Vec::<(&str, &str)>::new();
    let success = solve(&mut candidates, &mut solution);
    assert!(success);

    solution.sort();
    solution.iter()
        .map(|(_, ingredient)| ingredient)
        .join(",")
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), "mxmxvkd,sqjhc,fvjkl");
    assert_eq!(part2(&aoc::input()), "fsr,skrxt,lqbcg,mgbv,dvjrrkv,ndnlm,xcljh,zbhp");
}

fn main() {
    aoc::main(part1, part2);
}
