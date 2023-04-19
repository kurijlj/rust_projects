use std::fmt;
use rand::Rng;

fn main() {
    let suites = [Suite::Clubs, Suite::Diamonds, Suite::Hearts, Suite::Spades];
    let mut deck: Vec<Card> = Vec::new();
    let mut stack_idxs: [u8; 52] = [0; 52];

    let mut cntr: usize = 52;
    while 0 < cntr {
        let i: u8 = rand::thread_rng().gen_range(1..=52);
        if !contains(&stack_idxs[..], i) {
            stack_idxs[cntr-1] = i;
            cntr = cntr - 1;
        }
    }

    let mut i: usize = 1;
    while 52 >= i {
        let suite_idx = (stack_idxs[i-1] - 1) / 13;
        let rank = ((stack_idxs[i-1] - 13*suite_idx) + 1) as u8;

        deck.push(Card::new(&suites[suite_idx as usize], rank));

        i = i + 1;
    }

    for card in deck {
        println!("{}", card.to_string());
    }

}

fn contains(a: &[u8], v: u8) -> bool {
    if 1 == a.len() {
        if a[0] == v {
            true
        }

        else {
            false
        }
    }

    else {
        let s: usize = (a.len() / 2) as usize;
        contains(&a[0..s], v) || contains(&a[s..], v)
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

pub struct Card<'a> {
    suite: &'a Suite,
    rank: u8,
}


impl<'a> Card<'a> {
    pub fn new(s: &Suite, r: u8) -> Card {
        Card {
            suite: s,
            rank: r,
        }
    }
}

impl<'a> fmt::Display for Card<'a> {
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

