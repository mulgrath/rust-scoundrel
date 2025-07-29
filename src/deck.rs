use crate::card::{Card, Suit, Rank};
use rand::seq::SliceRandom;


pub struct Deck {
    cards: Vec<Card>,
    room: Vec<Card>,
    room_clear: bool,
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

        let room = Vec::with_capacity(4);
        Deck {cards, room, room_clear: false}
    }

    pub fn room_clear(&self) -> bool {
        self.room_clear
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn populate_room(&mut self) {
        // Draw the top 4 cards from the deck and move them to the room
        self.room = self.cards.drain(self.cards.len() - 4..).collect();
        self.room_clear = false;
    }

    pub fn get_deck_size(&self) -> usize {
        self.cards.len()
    }

    pub fn escape_room(&mut self) {
        // Reserve the size of the room to the deck so we avoid repeated reallocations
        self.cards.reserve(self.room.len());
        // Shift existing elements of the deck to the right by the room size while also draining
        // the room of cards.
        self.cards.splice(0..0, self.room.drain(..));
        // Set the room status to clear so the turn will end and the new room will be populated.
        self.room_clear = true;
    }

    pub fn get_room(&self) -> &[Card] {
        &self.room
    }

    pub fn print_deck(&self) {
        for card in &self.cards {
            println!("{}", card);
        }
        println!("There are {} cards in the deck.", self.cards.len());
    }

    pub fn print_room(&self) {
        println!("=== Room ===");
        for card in &self.room {
            println!("{}", card);
        }
        println!("============");
    }
}