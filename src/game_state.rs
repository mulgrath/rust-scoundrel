use crate::deck::Deck;
use crate::player::PlayerState;

pub struct GameState {
    player: PlayerState,
    deck: Deck,
    final_score: i32,
}

impl GameState {
    pub fn new() -> GameState {
        let player = PlayerState::new(20, 0, 0);
        let mut deck = Deck::new();
        deck.shuffle();
        GameState { player, deck, final_score: 0 }
    }

    pub fn print_deck(&self) {
        self.deck.print_deck();
    }

    pub fn print_player(&self) {
        self.player.print_player();
    }
}