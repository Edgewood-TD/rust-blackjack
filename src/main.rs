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
        println!(" got {:?}", card.clone());
    }
}
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub current_hand: Hand,
    pub balance: u32,
}
impl Player {
    pub fn new(_name: String) -> Self {
        Player {
            name: _name,
            current_hand: Hand::new_hand(),
            balance: 10000,
        }
    }
    pub fn add(&mut self, b: u32) {
        self.balance += b;
    }
    pub fn sub(&mut self, b: u32) {
        self.balance -= b;
    }
}
pub struct Game {
    pub dealer: Player,
    pub player: Player,
}
impl Game {
    pub fn new<'a>(player: Player) -> Self {
        Game {
            player,
            dealer: Player::new("Dealer".to_string()),
        }
    }

    pub fn start_game(&mut self) -> Option<&Player> {
        let ref mut deck = Deck::new_deck();
        deck.deal(&mut self.dealer.current_hand);
        deck.deal(&mut self.player.current_hand);
        deck.deal(&mut self.dealer.current_hand);
        deck.deal(&mut self.player.current_hand);
        loop {
            self.display_point();
            println!("Want more cards? (y,n)");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => match input.trim() {
                    "y" => {
                        deck.deal(&mut self.player.current_hand);
                    }
                    "n" => break,
                    _ => {
                        continue;
                    }
                },
                Err(error) => println!("error: {}", error),
            }
            if &self.player.current_hand.points() > &21 {
                println!("You have {} points", &self.player.current_hand.points());
                println!("You busted!");
                return Some(&self.dealer);
            }
        }
        println!("===========Result=============");
        if let Some(_) = self.dealer_turn(deck) {
            self.display_point();
            println!("Player Win!");
            println!("=============================");
            return Some(&self.player);
        } else {
            self.display_point();
            println!("Dealer Win!");
            println!("=============================");
            return Some(&self.dealer);
        }
    }
    fn dealer_turn(&mut self, deck: &mut Deck) -> Option<&Player> {
        println!("======Dealer Getting Cards======");
        while &self.dealer.current_hand.points() <= &16 {
            let _ = &mut deck.deal(&mut self.dealer.current_hand);
            println!(
                "Dealer have {} points",
                &mut self.dealer.current_hand.points()
            );
        }
        if &self.dealer.current_hand.points() > &21 {
            println!("Dealer have {} points", &self.dealer.current_hand.points());
            println!("You have {} points", &self.player.current_hand.points());
            println!("You Won!");
            return Some(&self.player);
        } else {
            None
        }
    }
    fn display_point(&self) {
        println!("You have {} points", self.player.current_hand.points());
        println!("Dealer have {} points", self.dealer.current_hand.points());
    }
}
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn new_hand() -> Self {
        Hand { cards: Vec::new() }
    }
    pub fn points(&self) -> u32 {
        let all_num: Vec<u32> = self
            .cards
            .iter()
            .map(|card| cmp::min(10, card.number) as u32)
            .collect();
        all_num.iter().sum()
    }
}
fn main() {
    loop {
        println!("Enter your bet to start , or e to exit");
        let mut input = String::new();
        let player = Player::new("Eric".to_string());
        let mut game = Game::new(player);
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(ok) => {
                    if let Some(winner) = game.start_game() {
                        match winner {
                            player => {}
                            _ => {}
                        }
                    }
                }
                _ => {
                    break;
                }
            },
            Err(error) => {
                println!("error: {}", error);
                ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
