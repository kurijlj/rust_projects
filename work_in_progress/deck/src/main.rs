// ============================================================================
//
// deck - A simple card shuffling program
//
//  Copyright (C) 2023 Ljubomir Kurij <ljubomir_kurij@protonmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// ============================================================================


// ============================================================================
//
// 2023-04-21 Ljubomir Kurij <ljubomir_kurij@protonmail.com>
//
// * main.rs: created.
//
// ============================================================================


// ============================================================================
// Used libraries section
// ============================================================================
use std::fmt;
use rand::Rng;
use std::option::Option;


// ============================================================================
// Main function section
// ============================================================================

fn main() {
    let mut cs1 = Deck::new(false);
    let mut cs2 = Deck::new(true);

    while !cs1.is_empty() {
        println!("{}", cs1.pick_card().unwrap());
    }
}


// ============================================================================
// User defined types section
// ============================================================================

// ----------------------------------------------------------------------------
//
// # Type "Suite"
//
// Represents a collection of playing cards suites.
//
// ----------------------------------------------------------------------------
pub enum Suite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

// ----------------------------------------------------------------------------
//
// # Trait "Suite::Display"
//
// Implements the `Display` trait for the `Suite` enum.
//
// The `Display` trait implementation for the `Suite` enum allows instances of
// the `Suite` enum to be formatted as strings using the `write!` macro.
// The implementation returns the name of the suite as a string.
//
// # Example
//
// ```
// let suite = Suite::Hearts;
// assert_eq!(format!("{}", suite), "Hearts");
// ```
//
// ----------------------------------------------------------------------------
impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suite::Clubs =>    "Clubs",
                Suite::Diamonds => "Diamonds",
                Suite::Hearts =>   "Hearts",
                Suite::Spades =>   "Spades",
            }
        )
    }
}

// ----------------------------------------------------------------------------
//
// # Function "get_suite"
//
// Returns the `Suite` enum variant corresponding to the given index,
// if it exists.
//
// The `get_suite` function returns an `Option` that contains the `Suite` enum
// variant that corresponds to the given index, if the index is in the range
// [0, 3]. If the index is not in the range [0, 3], the function returns `None`.
//
// # Arguments
//
// * `idx`: An `usize` that represents the index of the `Suite` enum variant
//          to retrieve.
//
// # Returns
//
// An `Option` that contains the `Suite` enum variant that corresponds to the
// given index, if it exists. If the index is not in the range [0, 3],
// the function returns `None`.
//
// # Example
//
// ```
// assert_eq!(get_suite(0), Some(Suite::Clubs));
// assert_eq!(get_suite(1), Some(Suite::Diamonds));
// assert_eq!(get_suite(2), Some(Suite::Hearts));
// assert_eq!(get_suite(3), Some(Suite::Spades));
// assert_eq!(get_suite(4), None);
// assert_eq!(get_suite(100), None);
// ```
//
// # Notes
//
// The `get_suite` function is a utility function that is used to retrieve
// the `Suite` enum variant that corresponds to a given index. The function
// returns an `Option` that contains the `Suite`variant if it exists, or `None`
// otherwise. The use of an `Option` allows the caller to handle the case where
// the variant does not exist in a safe and concise way.
//
// ----------------------------------------------------------------------------
fn get_suite(idx: usize) -> Option<Suite> {
    if 4 > idx {
        match idx {
            0 => Some(Suite::Clubs),
            1 => Some(Suite::Diamonds),
            2 => Some(Suite::Hearts),
            3 => Some(Suite::Spades),
            _ => todo!(),
        }
    } else {
        None
    }
}


// ----------------------------------------------------------------------------
//
// # Class "Card"
//
// Represents one playing card from a Frenc-suited deck of cards.
//
// ----------------------------------------------------------------------------
pub struct Card {
    suite: Suite,
    rank: u8,
}

// ----------------------------------------------------------------------------
//
// # Method "new"
//
// Creates a new `Card` instance with the given `Suite` and `rank`.
//
// # Arguments
//
// * `s` - The `Suite` of the card.
// * `r` - The rank of the card. The rank is an unsigned 8-bit integer that
//         represents the numeric value of the card. Ace is represented by
//         1, numbers 2 to 10 are represented by their respective values, and
//         face cards (Jack, Queen, King) are represented by 11, 12, and 13,
//         respectively.
//
// # Returns
//
// A new `Card` instance with the specified `Suite` and `rank`.
//
// # Example
//
// ```
// use my_crate::Card;
// use my_crate::Suite;
//
// let card = Card::new(Suite::Hearts, 5);
//
// assert_eq!(card.suite, Suite::Hearts);
// assert_eq!(card.rank, 5);
// ```
//
// ----------------------------------------------------------------------------
impl Card {
    pub fn new(s: Suite, r: u8) -> Card {
        Card {
            suite: s,
            rank: r,
        }
    }
}

// ----------------------------------------------------------------------------
//
// # Trait "Card::Display"
//
// A trait implementation for displaying a `Card` struct in a user-friendly way.
//
// # Examples
//
// ```
// let card = Card {
//     rank: 11,
//     suite: "Spades".to_string(),
// };
//
// assert_eq!(format!("{}", card), "Ace of Spades");
// ```
//
// # Errors
//
// This implementation returns a `fmt::Result` type that indicates whether
// the formatting operation was successful or not. If an error occurs during
// the formatting operation, a `fmt::Error` type is returned. This error type
// can be used to handle errors that occur during the formatting operation.
//
// # Panics
//
// This implementation does not panic under any circumstance. However,
// the output string may not be what the user expects if the `rank` or `suite`
// fields are not set correctly. It is the responsibility of the user to ensure
// that the `rank` and `suite` fields of a `Card` instance are set to valid
// values before using the `Display` trait implementation.
//
// ----------------------------------------------------------------------------
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let other_rank_str = &self.rank.to_string();

        write!(
            f,
            "{} of {}",
            match self.rank {
                11 => "Ace",
                12 => "Jack",
                13 => "Queen",
                14 => "King",
                _other => other_rank_str
            },
            self.suite
        )
    }
}


// ----------------------------------------------------------------------------
//
// # Class "Deck"
//
// Represents a Frenc-suited deck of cards.
//
// ----------------------------------------------------------------------------
pub struct Deck {
    cards: Vec<Card>,
    shuffled: bool,
}

// ----------------------------------------------------------------------------
//
// # Class "Deck" implementation
//
// ----------------------------------------------------------------------------
impl Deck {

// ----------------------------------------------------------------------------
//
// # Method "new"
//
// Creates a new `Deck` instance, consisting of 52 standard playing cards.
//
// This method takes a boolean `shuffle` as input, which indicates whether the
// newly created deck should be shuffled or not. The method creates a vector
// of `Card` instances by iterating through the possible combinations of
// `suite` and `rank` values for a standard deck of cards. Each `Card` instance
// is then added to the vector.
//
// # Arguments
//
// * `shuffle` - A boolean value that indicates whether the newly created deck
//               should be shuffled or not.
//
// # Examples
//
// ```
// use deck_of_cards::Deck;
//
// let deck1 = Deck::new(false);
// let deck2 = Deck::new(true);
// ```
//
// # Returns
//
// A new `Deck` instance with the `cards` vector consisting of 52 `Card`
// instances and the `shuffled` boolean set to the value of the `shuffle`
// parameter.
//
// # Panics
//
// This method does not panic under any circumstance. However, if the `shuffle`
// parameter is not a boolean value, unexpected behavior may occur. It is the
// responsibility of the user to ensure that the `shuffle` parameter is a
// boolean value before calling the `new()` method.
//
// ----------------------------------------------------------------------------
    pub fn new(shuffle: bool) -> Deck {
        let mut stack: Vec<Card> = Vec::new();

        let mut i: usize = 52;
        while 1 <= i {
            let suite_idx: usize = (i - 1)/13;
            let rank: usize = (i - 13*suite_idx) + 1;

            stack.push(Card::new(get_suite(suite_idx).unwrap(), rank as u8));

            i = i - 1;
        }

        Deck {
            cards: stack,
            shuffled: shuffle,
        }
    }

// ----------------------------------------------------------------------------
//
// # Method "shuffle"
//
// Shuffle the given deck of cards. This affects the given deck of cards in
// the way that taking the next card from the dack will be at random.
//
// # Arguments
//
// This method takes no arguments.
//
// # Examples
//
// ```
// use deck_of_cards::{Deck, Card};
//
// let mut deck = Deck::new(false);
// deck.shuffle()
// ```
//
// # Returns
//
// The method does not return any value.
//
// # Panics
//
// This method does not panic under any circumstance.
//
// ----------------------------------------------------------------------------
    pub fn shuffle(&mut self) {
        self.shuffled = true;
    }

// ----------------------------------------------------------------------------
//
// # Method "pick_card"
//
// Picks a card at random from the deck and returns it.
//
// This method removes a single `Card` instance from the deck and returns it
// to the caller. If the deck is empty, `None` is returned instead. If the deck
// is shuffled, a card is selected at random using the `rand` crate
// and returned. If the deck is not shuffled, the top card of the deck is
// returned.
//
// # Arguments
//
// This method takes no arguments.
//
// # Examples
//
// ```
// use deck_of_cards::{Deck, Card};
//
// let mut deck = Deck::new(true);
//
// let card = deck.pick_card();
// ```
//
// # Returns
//
// An `Option<Card>` instance representing the card that was picked from the
// deck. If the deck is empty, `None` is returned instead.
//
// # Panics
//
// This method does not panic under any circumstance. However, if the `Deck`
// instance is not properly initialized or if there is an issue with the `rand`
// crate, unexpected behavior may occur.
//
// ----------------------------------------------------------------------------
    pub fn pick_card(&mut self) -> Option<Card> {
        if !self.is_empty() {
            if self.shuffled {
                let left = self.cards.len() - 1;
                if 1 < left {
                    let i = rand::thread_rng().gen_range(1..=left);
                    Some(self.cards.remove(i))
                } else {
                    self.cards.pop()
                }
            } else {
                self.cards.pop()
            }
        } else {
            None
        }
    }

// ----------------------------------------------------------------------------
//
// # Method "cards_left"
//
// Returns the number of cards left in the deck.
//
// # Arguments
//
// This method takes no arguments.
//
// # Examples
//
// ```
// use deck_of_cards::{Deck, Card};
//
// let mut deck = Deck::new(false);
// deck.cards_left()
// ```
//
// # Returns
//
// A usize value indicating the number of cards left in the deck.
//
// # Panics
//
// This method does not panic under any circumstance.
//
// ----------------------------------------------------------------------------
    pub fn cards_left(&self) -> usize {
        self.cards.len()
    }

// ----------------------------------------------------------------------------
//
// # Method "is_empty"
//
// Returns whether are any cards left in the deck.
//
// # Arguments
//
// This method takes no arguments.
//
// # Examples
//
// ```
// use deck_of_cards::{Deck, Card};
//
// let mut deck = Deck::new(false);
// deck.is_empty()
// ```
//
// # Returns
//
// A boolean value indicating whether are any card left in the deck or not. If
// there are still cards in the deck it returns 'false', otherwise
// it returns 'true'.
//
// # Panics
//
// This method does not panic under any circumstance.
//
// ----------------------------------------------------------------------------
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
