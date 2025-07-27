use crate::card::{Card, Suit, Rank};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        for &suit in &[Suit::Clubs, Suit::Spades] {
            for &rank in &[Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine,
                Rank::Eight, Rank::Seven, Rank::Six, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
                cards.push(Card::new(suit, rank));
            }
        }

        // Scoundrel rules dictate that we exclude the red faces and Aces from the deck
        for &suit in &[Suit::Diamonds, Suit::Hearts] {
            for &rank in &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten] {
                cards.push(Card::new(suit, rank));
            }
        }

        Deck {cards}
    }

    pub fn shuffle(&mut self) {

    }

    pub fn print_deck(&self) {
        for card in &self.cards {
            println!("{}", card);
        }
    }
}