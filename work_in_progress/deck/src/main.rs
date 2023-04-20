use std::fmt;
use rand::Rng;
use std::option::Option;

fn main() {
    let mut cs1 = Deck::new(false);
    let mut cs2 = Deck::new(true);

    while !cs1.is_empty() {
        println!("{}", cs1.pick_card().unwrap());
    }
}

pub enum Suite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

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

pub struct Card {
    suite: Suite,
    rank: u8,
}


impl Card {
    pub fn new(s: Suite, r: u8) -> Card {
        Card {
            suite: s,
            rank: r,
        }
    }
}

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

pub struct Deck {
    cards: Vec<Card>,
    shuffled: bool,
}

impl Deck {
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

    pub fn shuffle(&mut self) {
        self.shuffled = true;
    }

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

    pub fn cards_left(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
