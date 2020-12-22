use packed_simd::{shuffle, Simd};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;

type Card = u8;

const NO_CARD: Card = 0;

// There are 50 cards. We can hold at most DECK_SIZE - 1 and it must be a power of two.
const DECK_SIZE: usize = 64;

type Cards = Simd<[Card; DECK_SIZE]>;

// Like VecDeque but fixed size, so no indirection needed.
#[derive(Clone, Hash)]
struct Deck {
    cards: Cards,
}

const NO_CARDS: Cards = Cards::splat(NO_CARD);

const CARD_INDICES: Cards = Cards::new(
    00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

impl FromIterator<Card> for Deck {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Deck {
        let mut iter = iter.into_iter();
        let mut i = 0;
        let mut cards = NO_CARDS;
        while let Some(card) = iter.next() {
            cards = cards.replace(i, card);
            i += 1;
        }
        debug_assert!(i < DECK_SIZE - 1);
        Deck {
            cards,
        }
    }
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards == NO_CARDS
    }

    fn len(&self) -> usize {
        self.cards.ne(NO_CARDS).bitmask().count_ones() as usize
    }

    fn iter(&self) -> DeckIterator {
        DeckIterator {
            cards: self.cards,
        }
    }

    fn push_back(&mut self, card: Card) {
        debug_assert!(self.len() < DECK_SIZE - 1);
        self.cards = self.cards.replace(self.len(), card);
    }

    fn pop_front(&mut self) -> Card {
        let card = self.cards.extract(0);
        debug_assert!(card != NO_CARD);
        self.cards = drop_first(self.cards);
        card
    }

    fn top(&self, count: usize) -> Deck {
        debug_assert!(count <= self.len());
        let cards = CARD_INDICES
            .lt(Cards::splat(count as u8))
            .select(self.cards, NO_CARDS);
        Deck {
            cards,
        }
    }
}

fn drop_first(cards: Cards) -> Cards {
    debug_assert!(cards.extract(0) != NO_CARD);
    debug_assert!(cards.extract(DECK_SIZE - 1) == NO_CARD);
    shuffle!(cards, [
        01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 63
    ])
}

impl Debug for Deck {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for i in 0..self.len() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self.cards.extract(i))?;
        }
        Ok(())
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Deck) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Deck {}

struct DeckIterator {
    cards: Cards,
}

impl Iterator for DeckIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        if self.cards == NO_CARDS {
            None
        } else {
            let result = Some(self.cards.extract(0));
            self.cards = drop_first(self.cards);
            result
        }
    }
}

type Decks = [Deck; 2];

fn parse(input: &str) -> Decks {
     let mut decks = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.parse::<Card>().unwrap())
                .collect::<Deck>()
        });
     [decks.next().unwrap(), decks.next().unwrap()]
}

fn score(deck: &Deck) -> u64 {
    deck.iter()
        .collect::<Vec<_>>() // Too lazy to implement DoubleEndedIterator for DeckIterator.
        .iter()
        .rev()
        .zip(1..)
        .map(|(&c, i)| c as u64 * i as u64)
        .sum()
}

fn part1(input: &str) -> u64 {
    let mut decks = parse(input);

    while !decks[0].is_empty() && !decks[1].is_empty() {
        let card0 = decks[0].pop_front();
        let card1 = decks[1].pop_front();
        if card0 > card1 {
            decks[0].push_back(card0);
            decks[0].push_back(card1);
        } else {
            debug_assert!(card0 < card1);
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

fn recursive_game(mut decks: Decks) -> (usize, Decks) {
    let mut prev_states = HashSet::with_capacity(512);
    
    while !decks[0].is_empty() && !decks[1].is_empty() {
        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly
        // ends in a win for player 1.
        if !prev_states.insert(decks.clone()) {
            return (0, decks);
        }
        // Otherwise, this round's cards must be in a new configuration; the players begin the
        // round by each drawing the top card of their deck as normal.
        let card0 = decks[0].pop_front();
        let card1 = decks[1].pop_front();
        let round_winner = if decks[0].len() as Card >= card0 && decks[1].len() as Card >= card1 {
            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat.
            //
            // To play a sub-game of Recursive Combat, each player creates a new deck by making a
            // copy of the next cards in their deck (the quantity of cards copied is equal to the
            // number on the card they drew to trigger the sub-game).
            let (subgame_winner, _) = recursive_game([
                decks[0].top(card0 as usize),
                decks[1].top(card1 as usize),
            ]);
            subgame_winner
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to
            // recurse; the winner of the round is the player with the higher-value card.
            if card0 > card1 {
                0
            } else {
                debug_assert!(card0 < card1);
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
        debug_assert!(decks[0].is_empty());
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
    assert_eq!(part2(&aoc::example(3)), 105);
    assert_eq!(part2(&aoc::input()), 35055);
}

fn main() {
    aoc::main(part1, part2);
}
