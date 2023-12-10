use std::cmp::Ordering;

use crate::file::read_lines;

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    symbol: char,
}

impl Card {
    pub fn build(symbol: char)->Card {
        Card {
            symbol
        }
    }

    fn value(&self)-> u8 {
        match self.symbol {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => {
                println!("ERROR symbol");
                0
            },
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Eq for Card { }

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    kind: u8,
    str: String
}

impl Hand {
    pub fn build(cards: Vec<Card>, bid: u32, str: String) -> Self {
        let kind = Self::value(cards.clone());
        Hand {
            cards,
            bid,
            kind,
            str
        }
    }

    fn value(cards: Vec<Card>)-> u8 {
        let mut hist = vec![1; 5];
        let mut i = 0;
        while i < 5 {
            let mut j = i + 1;
            while j < 5 {
                if cards[i] == cards[j] {
                    hist[j] += hist[i];
                    hist[i] = 0;
                }
                j += 1;
            }
            i+= 1;
        }
        println!("{:?}", hist);
        if hist[4] == 5 {
            6
        } else if hist[4] == 4 || hist[3] == 4 {
            5
        } else if hist[4] == 3 && hist[3] == 2 {
            4
        } else if hist[4] == 2 && hist[3] == 3 {
            4
        } else if hist[4] == 3 || hist[3] == 3 || hist[2] == 3 {
            3
        } else if hist.clone().into_iter().filter(|&x| x == 2).collect::<Vec<i32>>().len() == 2 {
            2
        } else if hist.into_iter().filter(|&x| x == 2).collect::<Vec<i32>>().len() == 1 {
            1
        } else {
            0
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_cmp = self.kind.cmp(&other.kind);
        if kind_cmp != Ordering::Equal {
            return kind_cmp;
        }
        let mut i = 0;
        while i < 5 {
            let card_cmp = self.cards[i].cmp(&other.cards[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
            i += 1;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn aoc7() {
    let mut hands = vec![];
    if let Ok(lines) = read_lines("inputs/aoc6.txt") {
        for l in lines {
            if let Ok(ip) = l {
                //println!("{}", ip);
                let mut cards = vec![];
                let mut it = ip.chars();
                loop {
                    if let Some(c) = it.next() {
                        if c == ' ' {
                            break;
                        } else {
                            cards.push(Card::build(c));
                        }
                    } else {
                        break;
                    }
                }
                let bid:String = it.collect();
                hands.push(Hand::build(cards, bid.parse().unwrap(), ip));
                println!("{:?}", hands[hands.len()-1].str);
                /*
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("can not read user input");
                    */
            }
        }
    } else {
        println!("file not found")
    }
    hands.sort();
    let mut i = 1;
    let mut result = 0;
    for h in hands {
        result += i * h.bid;
        i += 1;
    }
    println!("{:?}", result);
}
