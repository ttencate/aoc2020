use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> Vec<VecDeque<u64>> {
     input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.parse::<u64>().unwrap())
                .collect::<VecDeque<u64>>()
        })
        .collect::<Vec<VecDeque<u64>>>()
}

fn score(deck: &VecDeque<u64>) -> u64 {
    deck.iter()
        .rev()
        .zip(1..)
        .map(|(&c, i)| c * i)
        .sum()
}

fn part1(input: &str) -> u64 {
    let mut decks = parse(input);

    while !decks[0].is_empty() && !decks[1].is_empty() {
        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();
        if card0 > card1 {
            decks[0].push_back(card0);
            decks[0].push_back(card1);
        } else {
            assert!(card0 < card1);
            decks[1].push_back(card1);
            decks[1].push_back(card0);
        }
    }

    score(&decks[0]) + score(&decks[1])
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 306);
    assert_eq!(part1(&aoc::input()), 33098);
}

fn recursive_game(mut decks: Vec<VecDeque<u64>>) -> (usize, Vec<VecDeque<u64>>) {
    let mut prev_states = HashSet::new();
    
    while !decks[0].is_empty() && !decks[1].is_empty() {
        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly
        // ends in a win for player 1.
        if !prev_states.insert(decks.clone()) {
            return (0, decks);
        }
        // Otherwise, this round's cards must be in a new configuration; the players begin the
        // round by each drawing the top card of their deck as normal.
        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();
        let round_winner = if decks[0].len() as u64 >= card0 && decks[1].len() as u64 >= card1 {
            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat.
            //
            // To play a sub-game of Recursive Combat, each player creates a new deck by making a
            // copy of the next cards in their deck (the quantity of cards copied is equal to the
            // number on the card they drew to trigger the sub-game).
            let (winner, _) = recursive_game(vec![
                decks[0].iter().take(card0 as usize).copied().collect::<VecDeque<u64>>(),
                decks[1].iter().take(card1 as usize).copied().collect::<VecDeque<u64>>(),
            ]);
            winner
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to
            // recurse; the winner of the round is the player with the higher-value card.
            if card0 > card1 {
                0
            } else {
                assert!(card0 < card1);
                1
            }
        };
        match round_winner {
            0 => {
                decks[0].push_back(card0);
                decks[0].push_back(card1);
            },
            1 => {
                decks[1].push_back(card1);
                decks[1].push_back(card0);
            },
            _ => panic!(),
        }
    }

    let game_winner = if decks[1].is_empty() {
        0
    } else {
        assert!(decks[0].is_empty());
        1
    };
    (game_winner, decks)
}

fn part2(input: &str) -> u64 {
    let decks = parse(input);

    let (winner, decks) = recursive_game(decks);

    score(&decks[winner])
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 291);
    assert_eq!(part2(&aoc::input()), 35055);
}

fn main() {
    aoc::main(part1, part2);
}
