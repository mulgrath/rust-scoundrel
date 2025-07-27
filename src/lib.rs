use std::fmt;

#[derive(Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _suit_str_unicode = match self {
            Suit::Hearts => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs => "\u{2663}",
            Suit::Spades => "\u{2660}",
        };

        let _suit_str_literal = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };

        write!(f, "{}", _suit_str_literal)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_str = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", rank_str)
    }
}

struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
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

    fn shuffle(&mut self) {

    }

    fn print_deck(&self) {
        for card in &self.cards {
            println!("{}", card);
        }
    }
}

struct PlayerState {
    health: i32,
    attack: i32,
    defense: i32,
}

impl PlayerState {
    fn new(health: i32, attack: i32, defense: i32) -> PlayerState {
        PlayerState {health, attack, defense}
    }

    fn print_player(&self) {
        println!("Health: {}, Attack: {}, Defense: {}", self.health, self.attack, self.defense);
    }
}

struct GameState {
    player: PlayerState,
    deck: Deck,
}

impl GameState {
    fn new() -> GameState {
        let player = PlayerState::new(20, 0, 0);
        let deck = Deck::new();
        GameState { player, deck }
    }

    fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }
}

pub fn run_game() {
    let mut game_state = GameState::new();

    game_state.shuffle_deck();
    game_state.deck.print_deck();
    game_state.player.print_player();
}