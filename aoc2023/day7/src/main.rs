use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<usize>,
    pub bid: usize,
    pub hand_type: usize,
}

fn find_type(cards: &str) -> usize {
    let m = cards
        .chars()
        .fold(HashMap::<char, usize>::with_capacity(5), |mut m, c| {
            m.entry(c).and_modify(|e| *e += 1).or_insert(1);
            m
        });

    let has_five = m.values().filter(|&c| *c == 5).count();
    let has_four = m.values().filter(|&c| *c == 4).count();
    let has_three = m.values().filter(|&c| *c == 3).count();
    let has_two = m.values().filter(|&c| *c == 2).count();

    if has_five == 1 {
        6
    } else if has_four == 1 {
        5
    } else if has_three == 1 && has_two == 1 {
        4
    } else if has_three == 1 && has_two == 0 {
        3
    } else if has_two == 2 {
        2
    } else if has_two == 1 {
        1
    } else {
        0
    }
}

fn find_type_joker(cards: &str) -> usize {
    let m = cards
        .chars()
        .fold(HashMap::<char, usize>::with_capacity(5), |mut m, c| {
            m.entry(c).and_modify(|e| *e += 1).or_insert(1);
            m
        });

    let normal_cards = m.iter().filter(|(&k, _)| k != 'J').map(|(_, &v)| v);
    let jokers = *m.get(&'J').unwrap_or(&0);

    let has_five = normal_cards.clone().filter(|&c| c == 5).count();
    let has_four = normal_cards.clone().filter(|&c| c == 4).count();
    let has_three = normal_cards.clone().filter(|&c| c == 3).count();
    let has_two = normal_cards.clone().filter(|&c| c == 2).count();
    
    if has_five == 1 {
        6
    } else if has_four == 1 {
        if jokers == 1 {
            6
        } else {
            5
        }
    } else if has_three == 1 && has_two == 1 {
        4
    } else if has_three == 1 && has_two == 0 {
        if jokers == 2 {
            6
        } else if jokers == 1 {
            5
        } else {
            3
        }
    } else if has_two == 2 {
        if jokers == 1 {
            4
        } else {
            2
        }
    } else if has_two == 1 {
        if jokers == 3 {
            6
        } else if jokers == 2 {
            5
        } else if jokers == 1 {
            3
        } else {
            1
        }
    } else {
        if jokers == 5 {
            6
        } else if jokers == 4 {
            6
        } else if jokers == 3 {
            5
        } else if jokers == 2 {
            3
        } else if jokers == 1 {
            1
        } else {
            0
        }
    }
}


fn card_to_number(c: char) -> usize {
    if c == 'A' {
        14
    } else if c == 'K' {
        13
    } else if c == 'Q' {
        12
    } else if c == 'J' {
        11
    } else if c == 'T' {
        10
    } else {
        c.to_digit(10).unwrap() as usize
    }
}

fn card_to_number_joker(c: char) -> usize {
    if c == 'J' {
        0
    } else {
        card_to_number(c)
    }
}

impl Hand {
    fn new(
        cards: &str,
        bid: usize,
        card_mapping_fn: fn(char) -> usize,
        find_hand_type_fn: fn(&str) -> usize,
    ) -> Self {
        Self {
            cards: cards.chars().map(|c| card_mapping_fn(c)).collect(),
            bid,
            hand_type: find_hand_type_fn(cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (&this, &other) in self.cards.iter().zip(other.cards.iter()) {
            if this < other {
                return Ordering::Less;
            }
            if other < this {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }

        for (&this, &other) in self.cards.iter().zip(other.cards.iter()) {
            if this < other {
                return Some(Ordering::Less);
            }
            if other < this {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .all(|(x, y)| x == y)
    }
}

impl Eq for Hand {}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let hands = file
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut l| {
            (
                l.next().unwrap(),
                l.next().unwrap().parse::<usize>().unwrap(),
            )
        });

    let mut hands1 = hands.clone().map(|l| {
        Hand::new(
            l.0,
            l.1,
            card_to_number,
            find_type,
        )
    }).collect::<Vec<Hand>>();
    hands1.sort_unstable();

    let mut hands2 = hands.clone().map(|l| {
        Hand::new(
            l.0,
            l.1,
            card_to_number_joker,
            find_type_joker,
        )
    }).collect::<Vec<Hand>>();
    hands2.sort_unstable();

    let part1: usize = hands1.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    let part2: usize = hands2.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();

    println!("Day7 part1 is {part1}");
    println!("Day7 part2 is {part2}");
}
