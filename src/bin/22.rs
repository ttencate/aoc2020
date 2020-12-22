use packed_simd::{shuffle, Simd};
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hash, Hasher};

type Card = u8;

const NO_CARD: Card = 0;

// There are 50 cards. We can hold at most DECK_SIZE - 1 and it must be a power of two.
const DECK_SIZE: usize = 64;

type Cards = Simd<[Card; DECK_SIZE]>;

#[derive(Clone, PartialEq, Eq)]
struct Decks {
    cards: Cards,
}

const NO_CARDS: Cards = Cards::splat(NO_CARD);

const CARD_INDICES_0: Cards = Cards::new(
    00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

const CARD_INDICES_1: Cards = Cards::new(
    63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48,
    47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32,
    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
    15, 14, 13, 12, 11, 10, 09, 08, 07, 06, 05, 04, 03, 02, 01, 00);

impl Decks {
    fn parse(input: &str) -> Decks {
        let mut blocks = input.split("\n\n");
        let cards_0 = blocks.next().unwrap()
            .lines().skip(1)
            .map(|line| line.parse::<Card>().unwrap())
            .enumerate()
            .fold(NO_CARDS, |cards, (i, card)| cards.replace(i, card));
        let cards_1 = blocks.next().unwrap()
            .lines().skip(1)
            .map(|line| line.parse::<Card>().unwrap())
            .enumerate()
            .fold(NO_CARDS, |cards, (i, card)| cards.replace(DECK_SIZE - 1 - i, card));
        Decks { cards: cards_0 | cards_1 }
    }

    #[cfg(test)]
    fn new(cards_0: &[Card], cards_1: &[Card]) -> Decks {
        let mut cards = [NO_CARD; 64];
        for (i, &card) in cards_0.iter().enumerate() {
            cards[i] = card;
        }
        for (i, &card) in cards_1.iter().enumerate() {
            debug_assert!(cards[DECK_SIZE - 1 - i] == NO_CARD);
            cards[DECK_SIZE - 1 - i] = card;
        }
        Decks { cards: Cards::from_slice_unaligned(&cards) }
    }

    fn is_any_empty(&self) -> bool {
        self.is_empty_0() || self.is_empty_1()
    }

    fn is_empty_0(&self) -> bool {
        NO_CARD == unsafe { self.cards.extract_unchecked(0) }
    }

    fn is_empty_1(&self) -> bool {
        NO_CARD == unsafe { self.cards.extract_unchecked(DECK_SIZE - 1) }
    }

    fn len_0(&self) -> u32 {
        self.cards.ne(NO_CARDS).bitmask().trailing_ones()
    }

    fn len_1(&self) -> u32 {
        self.cards.ne(NO_CARDS).bitmask().leading_ones()
    }

    fn cards_0(&self) -> Vec<Card> {
        let mut cards = [NO_CARD; DECK_SIZE];
        unsafe {
            self.cards.write_to_slice_unaligned_unchecked(&mut cards);
        }
        cards.iter().copied().take_while(|&card| card != NO_CARD).collect()
    }

    fn cards_1(&self) -> Vec<Card> {
        let mut cards = [NO_CARD; DECK_SIZE];
        unsafe {
            self.cards.write_to_slice_unaligned_unchecked(&mut cards);
        }
        cards.iter().copied().rev().take_while(|&card| card != NO_CARD).collect()
    }

    fn push_back_0(&mut self, a: Card, b: Card) {
        debug_assert!((self.len_0() as usize) + (self.len_1() as usize) < DECK_SIZE - 2);
        let idx = self.len_0() as usize;
        self.cards = self.cards.replace(idx, a).replace(idx + 1, b);
    }

    fn push_back_1(&mut self, a: Card, b: Card) {
        debug_assert!((self.len_0() as usize) + (self.len_1() as usize) < DECK_SIZE - 2);
        let idx = DECK_SIZE - 1 - self.len_1() as usize;
        self.cards = self.cards.replace(idx, a).replace(idx - 1, b);
    }

    fn pop_both(&mut self) -> (Card, Card) {
        let card_0 = unsafe { self.cards.extract_unchecked(0) };
        let card_1 = unsafe { self.cards.extract_unchecked(63) };
        debug_assert!(card_0 != NO_CARD);
        debug_assert!(card_1 != NO_CARD);
        let popped_0 = shuffle!(self.cards, [
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 63
        ]);
        let popped_1 = shuffle!(self.cards, [
            00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
        ]);
        self.cards = CARD_INDICES_0.lt(Cards::splat(self.len_0() as u8 - 1)).select(
            popped_0,
            CARD_INDICES_1.lt(Cards::splat(self.len_1() as u8 - 1)).select(popped_1, NO_CARDS));
        (card_0, card_1)
    }

    fn tops(&self, count_0: u8, count_1: u8) -> Decks {
        debug_assert!(count_0 as u32 <= self.len_0());
        debug_assert!(count_1 as u32 <= self.len_1());
        let mask =
            CARD_INDICES_0.lt(Cards::splat(count_0)) |
            CARD_INDICES_1.lt(Cards::splat(count_1));
        Decks { cards: mask.select(self.cards, NO_CARDS) }
    }
}

#[test]
fn test_deck_queries() {
    let empty_both = Decks::new(&[], &[]);
    assert!(empty_both.is_any_empty());
    assert!(empty_both.is_empty_0());
    assert!(empty_both.is_empty_1());
    assert_eq!(empty_both.len_0(), 0);
    assert_eq!(empty_both.len_1(), 0);
    assert_eq!(empty_both.cards_0(), Vec::<Card>::new());
    assert_eq!(empty_both.cards_1(), Vec::<Card>::new());

    let empty_0 = Decks::new(&[], &[42]);
    assert!(empty_0.is_any_empty());
    assert!(empty_0.is_empty_0());
    assert!(!empty_0.is_empty_1());
    assert_eq!(empty_0.len_0(), 0);
    assert_eq!(empty_0.len_1(), 1);
    assert_eq!(empty_0.cards_0(), Vec::<Card>::new());
    assert_eq!(empty_0.cards_1(), vec![42]);

    let empty_1 = Decks::new(&[42], &[]);
    assert!(empty_1.is_any_empty());
    assert!(!empty_1.is_empty_0());
    assert!(empty_1.is_empty_1());
    assert_eq!(empty_1.len_0(), 1);
    assert_eq!(empty_1.len_1(), 0);
    assert_eq!(empty_1.cards_0(), vec![42]);
    assert_eq!(empty_1.cards_1(), Vec::<Card>::new());

    let empty_neither = Decks::new(&[42], &[37]);
    assert!(!empty_neither.is_any_empty());
    assert!(!empty_neither.is_empty_0());
    assert!(!empty_neither.is_empty_1());
    assert_eq!(empty_neither.len_0(), 1);
    assert_eq!(empty_neither.len_1(), 1);
    assert_eq!(empty_neither.cards_0(), vec![42]);
    assert_eq!(empty_neither.cards_1(), vec![37]);

    let decks = Decks::new(&[1, 2, 3], &[4, 5, 6]);
    assert!(!decks.is_any_empty());
    assert!(!decks.is_empty_0());
    assert!(!decks.is_empty_1());
    assert_eq!(decks.len_0(), 3);
    assert_eq!(decks.len_1(), 3);
    assert_eq!(decks.cards_0(), vec![1, 2, 3]);
    assert_eq!(decks.cards_1(), vec![4, 5, 6]);
}

fn score(cards: Vec<Card>) -> u64 {
    cards
        .into_iter()
        .rev()
        .zip(1..)
        .map(|(c, i)| c as u64 * i as u64)
        .sum()
}

fn part1(input: &str) -> u64 {
    let mut decks = Decks::parse(input);

    while !decks.is_any_empty() {
        let (card_0, card_1) = decks.pop_both();
        if card_0 > card_1 {
            decks.push_back_0(card_0, card_1);
        } else {
            debug_assert!(card_0 < card_1);
            decks.push_back_1(card_1, card_0);
        }
    }

    score(decks.cards_0()) + score(decks.cards_1())
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 306);
    assert_eq!(part1(&aoc::input()), 33098);
}

impl Hash for Decks {
    fn hash<H: Hasher>(&self, h: &mut H) {
        unsafe {
            std::mem::transmute::<&Cards, &[u8; 64]>(&self.cards).hash(h)
        }
    }
}

fn recursive_game(mut decks: Decks) -> (usize, Decks) {
    let mut prev_states = HashSet::with_capacity_and_hasher(
        512, BuildHasherDefault::<rustc_hash::FxHasher>::default());
    
    while !decks.is_any_empty() {
        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly
        // ends in a win for player 1.
        if !prev_states.insert(decks.clone()) {
            return (0, decks);
        }
        // Otherwise, this round's cards must be in a new configuration; the players begin the
        // round by each drawing the top card of their deck as normal.
        let (card_0, card_1) = decks.pop_both();
        let round_winner = if decks.len_0() as Card >= card_0 && decks.len_1() as Card >= card_1 {
            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat.
            //
            // To play a sub-game of Recursive Combat, each player creates a new deck by making a
            // copy of the next cards in their deck (the quantity of cards copied is equal to the
            // number on the card they drew to trigger the sub-game).
            let (subgame_winner, _) = recursive_game(decks.tops(card_0, card_1));
            subgame_winner
        } else {
            // Otherwise, at least one player must not have enough cards left in their deck to
            // recurse; the winner of the round is the player with the higher-value card.
            if card_0 > card_1 {
                0
            } else {
                debug_assert!(card_0 < card_1);
                1
            }
        };
        match round_winner {
            0 => decks.push_back_0(card_0, card_1),
            1 => decks.push_back_1(card_1, card_0),
            _ => panic!(),
        }
    }

    let game_winner = if decks.is_empty_1() {
        0
    } else {
        debug_assert!(decks.is_empty_0());
        1
    };
    (game_winner, decks)
}

fn part2(input: &str) -> u64 {
    let decks = Decks::parse(input);

    let (winner, decks) = recursive_game(decks);

    match winner {
        0 => score(decks.cards_0()),
        1 => score(decks.cards_1()),
        _ => panic!(),
    }
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
