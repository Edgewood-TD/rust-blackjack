use self::Suit::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{cmp, io};

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Diamonds = 0,
    Clubs = 1,
    Hearts = 2,
    Spades = 3,
}
impl Suit {
    pub fn iterator() -> impl Iterator<Item = Suit> {
        [Diamonds, Clubs, Hearts, Spades].iter().copied()
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub used_count: u16,
}
impl Deck {
    pub fn new_deck() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::iterator() {
            for i in 1..=13 {
                cards.push(Card {
                    number: i,
                    suit: suit,
                });
            }
        }
        cards.shuffle(&mut thread_rng());
        Deck {
            cards: cards,
            used_count: 0,
        }
    }
    pub fn print_cards(&self) {
        for card in &self.cards {
            println!("{:?}", card);
        }
        println!("{}", self.cards.len().to_string());
    }
    pub fn deal(&mut self, hand: &mut Hand) {
        let card = self.cards.pop().unwrap();

        hand.cards.push(card.clone());
        println!("{} got {:?}", hand.name, card.clone());
    }
}
#[derive(Debug)]
pub struct Hand {
    pub name: String,
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn new_hand(name: String) -> Self {
        Hand {
            name: name,
            cards: Vec::new(),
        }
    }
    pub fn points(&self) -> u32 {
        let allnum: Vec<u32> = self
            .cards
            .iter()
            .map(|card| cmp::min(10, card.number) as u32)
            .collect();
        allnum.iter().sum()
    }
}
fn main() {
    loop {
        println!("Enter s to start , e to exit");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "s" => play_game(),
                "e" => break,
                _ => {
                    continue;
                }
            },
            Err(error) => println!("error: {}", error),
        }
    }
}

fn play_game() {
    let ref mut player_a: Hand = Hand::new_hand("player".to_string());
    let ref mut dealer: Hand = Hand::new_hand("Dealer".to_string());
    let ref mut deck = Deck::new_deck();
    deck.deal(player_a);
    deck.deal(dealer);
    deck.deal(player_a);
    deck.deal(dealer);

    loop {
        println!("You have {} points", player_a.points());
        println!("Dealer have {} points", dealer.points());
        println!("Want more cards? (y,n)");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => match input.trim() {
                "y" => {
                    deck.deal(player_a);
                }
                "n" => break,
                _ => {
                    continue;
                }
            },
            Err(error) => println!("error: {}", error),
        }
        if player_a.points() > 21 {
            println!("You have {} points", player_a.points());
            println!("You busted!");
            return;
        }
    }
    println!("======Dealer Getting Cards======");
    while dealer.points() <= 16 {
        deck.deal(dealer);
        println!("Dealer have {} points", dealer.points());
    }
    println!("===========Result=============");
    if dealer.points() > 21 {
        println!("Dealer have {} points", dealer.points());
        println!("You have {} points", player_a.points());
        println!("You Won!");
        return;
    } else {
        println!("You have {} points", player_a.points());
        println!("Dealer have {} points", dealer.points());
        match player_a.points() > dealer.points() {
            true => println!("You Won!"),
            false => println!("You Lost!"),
        }
    }
    println!("=============================");
}
