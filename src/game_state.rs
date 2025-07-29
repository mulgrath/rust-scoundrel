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
    escape_cooldown: i32,
}

impl GameState {
    pub fn new() -> GameState {
        let player = PlayerState::new(20, 20, 0, 0);
        let mut deck = Deck::new();
        deck.shuffle();
        GameState { player, deck, final_score: 0, player_actions: vec![], can_escape: true, escape_cooldown: 0 }
    }

    pub fn start_turn(&mut self) {
        self.deck.populate_room();
        self.deck.print_room();
        // Have player choose to escape the room or select a card until the room is cleared...
        while !self.deck.room_clear() {
            self.process_player_turn();
        }
    }

    pub fn game_over(&mut self) -> bool {
        if (self.player.health() <= 0) {
            true
        }
        else if (self.deck.get_deck_size() == 0) {
            true
        }
        else {
            false
        }
    }

    fn process_player_turn(&mut self) {
        self.update_escape_status();
        self.get_available_actions();
        println!("Choose your action...");
        self.print_player_choices();
        let choice = GameState::get_player_choice("Choose your action: ", 1, self.player_actions.len());
        let chosen_action: &PlayerAction = self.player_actions.get(choice-1).unwrap();
        match chosen_action {
            PlayerAction::FightMonster(val) => {
                self.fight_monster(*val);
            },
            PlayerAction::DrinkPotion(val) => {
                let amt = self.player.get_potion_heal_amount(val);
                self.player.heal(amt);
            },
            PlayerAction::EquipWeapon(val) => {
                self.player.equip(*val);
            },
            PlayerAction::EscapeRoom => {
                self.escape_room();
            }
        }
    }

    fn print_player_choices(&self) {
        for (choice_num, action) in self.player_actions.iter().enumerate() {
            match action {
                PlayerAction::FightMonster(val) => {
                  println!("{}. Fight Monster ({val} Combat Power)", choice_num+1);
                },
                PlayerAction::DrinkPotion(val) => {
                    let amt = self.player.get_potion_heal_amount(val);
                    println!("{}. Drink Potion (+{amt} HP)", choice_num+1)
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
            io::stdin().read_line(&mut input).expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(num) if (min..=max).contains(&num) => return num,
                _ => {
                    println!("Invalid choice, enter a number between {} and {}", min, max);
                    continue;
                }
            };
        }
    }

    fn fight_monster(&mut self, val: i32) {
        println!("You entered combat with a monster of combat power {}", val);
    }

    /// Escaping the room can only be done before the player takes any action in the room.
    /// The escape cooldown is set to 2 so that in the next room when the escape status is checked,
    /// the counter will be decremented by one, which is still not finished cooling down.
    /// This prevents the player from escaping two rooms in a row.
    fn escape_room(&mut self) {
        println!("You fled the room...");
        self.escape_cooldown = 2;
        self.deck.escape_room();
    }

    fn update_escape_status(&mut self) {
        if self.escape_cooldown <= 0 {
            self.can_escape = true;
        }
        else {
            self.escape_cooldown -= 1;
            self.can_escape = false;
        }
    }

    pub fn print_deck(&self) {
        self.deck.print_deck();
    }

    pub fn print_player(&self) {
        self.player.print_player();
    }
}