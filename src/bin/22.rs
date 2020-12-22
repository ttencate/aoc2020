use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

type Card = u8;

// There are 50 cards. We can hold at most DECK_SIZE - 1 and it must be a power of two.
const DECK_SIZE: usize = 64;
const DECK_MASK: usize = DECK_SIZE - 1;

// Like VecDeque but fixed size, so no indirection needed.
#[derive(Clone)]
struct Deck {
    start: usize,
    end: usize,
    cards: [Card; DECK_SIZE],
}

impl FromIterator<Card> for Deck {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Deck {
        let mut iter = iter.into_iter();
        let mut i = 0;
        let mut cards = [0; DECK_SIZE];
        while let Some(card) = iter.next() {
            cards[i] = card;
            i += 1;
        }
        debug_assert!(i < DECK_SIZE - 1);
        Deck {
            start: 0,
            end: i,
            cards,
        }
    }
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.start == self.end
    }

    fn len(&self) -> usize {
        if self.start <= self.end {
            self.end - self.start
        } else {
            DECK_SIZE - self.start + self.end
        }
    }

    fn iter<'a>(&'a self) -> DeckIterator<'a> {
        DeckIterator {
            deck: self,
            i: self.start,
        }
    }

    fn push_back(&mut self, card: Card) {
        self.cards[self.end] = card;
        self.end = (self.end + 1) & DECK_MASK;
    }

    fn pop_front(&mut self) -> Card {
        let card = self.cards[self.start];
        self.start = (self.start + 1) & DECK_MASK;
        card
    }

    fn top(&self, count: usize) -> Deck {
        Deck {
            start: self.start,
            end: (self.start + count) & DECK_MASK,
            cards: self.cards.clone(),
        }
    }
}

impl Debug for Deck {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut i = self.start;
        let mut prepend_comma = false;
        while i != self.end {
            if prepend_comma {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self.cards[i])?;
            i = (i + 1) & DECK_MASK;
            prepend_comma = true;
        }
        Ok(())
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Deck) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut i = self.start;
        let mut j = other.start;
        while i != self.end {
            if self.cards[i] != other.cards[j] {
                return false;
            }
            i = (i + 1) & DECK_MASK;
            j = (j + 1) & DECK_MASK;
        }
        true
    }
}

impl Hash for Deck {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.len().hash(hasher);
        let mut i = self.start;
        while i != self.end {
            self.cards[i].hash(hasher);
            i = (i + 1) & DECK_MASK;
        }
    }
}

impl Eq for Deck {}

struct DeckIterator<'a> {
    deck: &'a Deck,
    i: usize,
}

impl<'a> Iterator for DeckIterator<'a> {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        if self.i == self.deck.end {
            None
        } else {
            let result = Some(self.deck.cards[self.i]);
            self.i = (self.i + 1) & DECK_MASK;
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
        .collect::<Vec<_>>() // TODO implement DoubleEndedIterator for DeckIterator
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
