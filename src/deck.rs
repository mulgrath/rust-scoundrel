use crate::card::{Card, Suit, Rank};
use rand::seq::SliceRandom;


pub struct Deck {
    cards: Vec<Card>,
    room: Vec<Card>,
    room_clear: bool,
    room_size: usize,
}

impl Deck {
    pub fn new(room_size: usize) -> Deck {
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

        let room = Vec::with_capacity(room_size);
        Deck {cards, room, room_clear: false, room_size}
    }

    pub fn room_clear(&self) -> bool {
        self.room_clear
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draw the top cards from the deck and move them to the room until it is full
    pub fn populate_room(&mut self) {
        let num_to_draw = self.room_size.saturating_sub(self.room.len());
        let to_take = num_to_draw.min(self.cards.len());
        let start = self.cards.len().saturating_sub(to_take);
        self.room.extend(self.cards.drain(start..));
        self.room_clear = false;
    }


    /// Returns true if there are no cards in the room or in the deck.
    /// Returns false otherwise.
    pub fn dungeon_cleared(&self) -> bool {
        if self.cards.len() == 0 {
            // If the last card in the game is a potion, end the game so the potion is added to the player's score.
            if self.room.len() == 1 && self.room.first().unwrap().suit() == Suit::Hearts {
                true
            }
            else if self.room.len() == 0 {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    pub fn remove_card_from_room(&mut self, idx: usize) {
        self.room.remove(idx);

        // If the player only has one card left in the room, the room is refilled and the remaining
        // card stays in the new room.
        // However, if there are no more cards in the deck, the player must empty the room completely
        if self.room.len() == 1 && self.cards.len() > 0 {
            self.room_clear = true;
        }
        else if self.room.len() == 0 && self.cards.len() == 0 {
            self.room_clear = true;
        }
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

    /// When the player dies, the final score is the sum of the remaining monsters' values minus the player's health.
    /// So loop over the remaining cards in the deck and room to get the total value of monsters.
    pub fn get_remaining_monsters(&self) -> i32 {
        let mut result = 0;

        for card in &self.cards {
            if card.suit() == Suit::Clubs || card.suit() == Suit::Spades {
                result += card.rank();
            }
        }

        for card in &self.room {
            if card.suit() == Suit::Clubs || card.suit() == Suit::Spades {
                result += card.rank();
            }
        }

        result
    }

    /// If the player wins, and the final card in the room is a potion, add that value to the score
    pub fn get_final_potion_bonus_score(&mut self) -> i32 {
        let mut result = 0;

        if self.room.len() == 1 {
            let card = &self.room.pop().unwrap();
            if card.suit() == Suit::Hearts {
                result += card.rank();
            }
        }

        result
    }
}