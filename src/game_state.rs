use crate::deck::Deck;
use crate::player::{PlayerAction, PlayerState};
use crate::card::{Suit};
use std::io::{self, Write};
use std::cmp;

pub struct GameState {
    player: PlayerState,
    deck: Deck,
    final_score: i32,
    player_actions: Vec<PlayerAction>,
    can_escape: bool,
}

impl GameState {
    pub fn new() -> GameState {
        let player = PlayerState::new(15, 0, 0);
        let mut deck = Deck::new();
        deck.shuffle();
        GameState { player, deck, final_score: 0, player_actions: vec![], can_escape: true }
    }

    pub fn start_turn(&mut self) {
        self.deck.populate_room();
        self.deck.print_room();
        // Have player choose to escape the room or select a card...
        self.process_player_turn();
    }

    fn process_player_turn(&mut self) {
        self.get_available_actions();
        println!("Choose your action...");
        self.print_player_choices();
        let choice = GameState::get_player_choice("Choose your action: ", 1, self.player_actions.len());
        println!("You chose: {choice}");
    }

    fn print_player_choices(&self) {
        for (choice_num, action) in self.player_actions.iter().enumerate() {
            match action {
                PlayerAction::FightMonster(val) => {
                  println!("{}. Fight Monster ({val} Combat Power)", choice_num+1);
                },
                PlayerAction::DrinkPotion(val) => {
                    let mut restore_amt = cmp::min(20 - ((*val) + self.player.health()), *val);
                    if restore_amt <=  0 {
                        restore_amt = *val + restore_amt;
                    }
                    println!("{}. Drink Potion (+{restore_amt} HP)", choice_num+1)
                },
                PlayerAction::EquipWeapon(val) => {
                    println!("{}. Equip Weapon ({val} Damage)", choice_num+1)
                },
                PlayerAction::EscapeRoom => {
                    println!("{}. Escape room", choice_num+1);
                }
            }
        }
    }

    fn get_available_actions(&mut self) {
        self.player_actions.clear();

        for card in self.deck.get_room() {
            if card.suit() == Suit::Diamonds {
                self.player_actions.push(PlayerAction::EquipWeapon(card.rank()));
            }
            else if card.suit() == Suit::Hearts {
                self.player_actions.push(PlayerAction::DrinkPotion(card.rank()));
            }
            else if card.suit() == Suit::Spades || card.suit() == Suit::Clubs {
                self.player_actions.push(PlayerAction::FightMonster(card.rank()));
            }
        }

        if self.can_escape {
            self.player_actions.push(PlayerAction::EscapeRoom);
        }
    }

    fn get_player_choice(prompt: &str, min: usize, max: usize) -> usize {
        loop {
            print!("{}: ", prompt);
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");;

            match input.trim().parse::<usize>() {
                Ok(num) if (min..=max).contains(&num) => return num,
                _ => {
                    println!("Invalid choice, enter a number between {} and {}", min, max);
                    continue;
                }
            };
        }
    }

    pub fn print_deck(&self) {
        self.deck.print_deck();
    }

    pub fn print_player(&self) {
        self.player.print_player();
    }
}