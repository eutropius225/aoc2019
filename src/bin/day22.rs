use std::io::{stdin, BufRead};
use itertools::Itertools;
use num::{Integer, BigInt, One, Zero};
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::str::FromStr;
use std::convert::TryInto;

type SpaceCard = usize;

enum Shuffle {
    Reverse,
    Increment(usize),
    Cut(i64),
}

impl FromStr for Shuffle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s.split(" ").collect_vec()[..] {
            ["deal", "into", "new", "stack"] => Shuffle::Reverse,
            ["deal", "with", "increment", n] => Shuffle::Increment(n.parse().unwrap()),
            ["cut", n] => Shuffle::Cut(n.parse().unwrap()),
            [..] => panic!("Unknown instruction: {}", s)
        })
    }
}

#[derive(Clone)]
struct Deck {
    count: usize,
    // x' = ax - b mod count
    a: BigInt,
    b: BigInt,
}

impl Debug for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Deck[i < {}] => ", self.count)?;
        if !self.a.is_zero() {
            if !self.a.is_one() {
                write!(f, "{}", self.a)?;
            }
            write!(f, "i")?;
        }
        if !self.b.is_zero() {
            if !self.a.is_zero() {
                write!(f, " + ")?;
            }
            write!(f, "{}", self.b)?;
        }
        Ok(())
    }
}

impl Deck {
    fn factory(count: usize) -> Self {
        Deck {
            count,
            a: 1.into(),
            b: 0.into(),
        }
    }

    fn shuffle(self, shuffle: &Vec<Shuffle>) -> Self {
        let mut deck = self;
        for s in shuffle.iter().rev() {
            match s {
                Shuffle::Reverse => deck = deck.deal_into_new_stack(),
                Shuffle::Increment(i) => deck = deck.deal_with_increment(*i),
                Shuffle::Cut(c) => deck = deck.cut(*c),
            }
        }
        deck
    }

    fn iter(&self) -> impl Iterator<Item=SpaceCard> + '_ {
        (0..self.len()).map(move |c| self.get(c))
    }

    fn deal_into_new_stack(self) -> Self {
        Deck {
            count: self.count,
            a: -self.a,
            b: self.count - self.b - 1,
        }
    }

    fn cut(self, n: i64) -> Self {
        Deck {
            count: self.count,
            a: self.a,
            b: (self.b + n) % self.count,
        }
    }

    fn deal_with_increment(self, n: usize) -> Self {
        let z = BigInt::from(n).modpow(&(self.count - 2).into(), &self.count.into());
        Deck {
            a: (self.a * &z) % self.count,
            b: (self.b * &z) % self.count,
            count: self.count,
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    fn modpow(self, exp: usize) -> Self {
        if exp == 0 {
            Deck {
                count: self.count,
                a: 1.into(),
                b: 0.into(),
            }
        } else if exp % 2 == 0 {
            // ah yes log(n)
            Deck {
                count: self.count,
                a: self.a.modpow(&2.into(), &self.count.into()),
                b: ((self.a + 1) * self.b) % self.count,
            }.modpow(exp / 2)
        } else {
            let a = self.a.clone();
            let b = self.b.clone();
            let count = self.count;
            let Deck { a: c, b: d, .. } = self.modpow(exp - 1);
            Deck {
                count,
                a: (&a * c) % count,
                b: (&a * d + b) % count,
            }
        }
    }

    fn get(&self, index: usize) -> SpaceCard {
        BigInt::from((&self.a * index as i64 + &self.b)
            .mod_floor(&self.len().into()))
            .try_into()
            .unwrap()
    }
}

const CARD_COUNT_1: usize = 10007;
const CARD_COUNT_2: usize = 119_315_717_514_047;
const REPEATS: usize = 101_741_582_076_661;

fn main() {
    let stdin = stdin();
    let shuffle = stdin
        .lock()
        .lines()
        .into_iter()
        .map(|r| r.unwrap())
        .map(|s| s.parse())
        .map(|r| r.unwrap())
        .collect_vec();

    let pos = Deck::factory(CARD_COUNT_1)
        .shuffle(&shuffle)
        .iter()
        .find_position(|&c| c == 2019)
        .unwrap()
        .0;
    println!("Position: {}", pos);

    let card = Deck::factory(CARD_COUNT_2)
        .shuffle(&shuffle)
        .modpow(REPEATS)
        .get(2020);
    println!("Card: {}", card);
}
