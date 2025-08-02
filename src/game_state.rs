use crate::deck::Deck;
use crate::player::{PlayerAction, PlayerState};
use crate::card::{Suit};
use std::io::{self, Write};

pub struct GameState {
    player: PlayerState,
    deck: Deck,
    player_actions: Vec<PlayerAction>,
    can_escape: bool,
    escape_cooldown: i32,
}

impl GameState {
    pub fn new() -> GameState {
        let player = PlayerState::new(20);
        let mut deck = Deck::new(4);
        deck.shuffle();
        GameState { player, deck, player_actions: vec![], can_escape: true, escape_cooldown: 0 }
    }

    pub fn enter_room(&mut self) {
        self.update_escape_status();
        self.deck.populate_room();
        self.player.remove_potion_cooldown();

        while !self.deck.room_clear() && self.player.health() > 0 {
            self.deck.print_room();
            self.player.print_player();
            self.process_player_turn();
        }
    }

    pub fn game_over(&mut self) -> bool {
        if self.player.health() <= 0 {
            println!("Game over! You have fallen in battle...");
            let final_score = self.player.health() - self.deck.get_remaining_monsters();
            println!("Your final score is: {}", final_score);
            true
        }
        else if self.deck.dungeon_cleared() {
            println!("Game Over! You have successfully cleared the dungeon!");
            let final_score = self.player.health() + self.deck.get_final_potion_bonus_score();
            println!("Your final score is: {}", final_score);
            true
        }
        else {
            false
        }
    }

    fn process_player_turn(&mut self) {
        self.get_available_actions();
        self.print_player_choices();
        let choice = GameState::get_player_choice("Enter action number", 1, self.player_actions.len()) - 1; // Subtract 1 to account for menu items starting at 1
        let chosen_action: &PlayerAction = self.player_actions.get(choice).unwrap();
        match chosen_action {
            PlayerAction::FightMonster(val) => {
                self.can_escape = false;
                let mut using_weapon = false;
                if self.player.can_use_weapon(*val) {
                    // Ask if the player wants to use their weapon or hands
                    self.print_combat_choices();
                    let choice = GameState::get_player_choice("Choose your weapon: ", 1, 2);
                    if choice == 1 {
                        using_weapon = true;
                    }
                }
                self.player.attack_monster(*val, using_weapon);
                self.deck.remove_card_from_room(choice);
            },
            PlayerAction::DrinkPotion(val) => {
                self.can_escape = false;
                let amt = self.player.get_potion_heal_amount(*val);
                self.player.heal(amt);
                self.deck.remove_card_from_room(choice);
            },
            PlayerAction::EquipWeapon(val) => {
                self.can_escape = false;
                self.player.equip(*val);
                self.deck.remove_card_from_room(choice);
            },
            PlayerAction::EscapeRoom => {
                self.escape_room();
            }
        }
    }

    fn print_combat_choices(&self) {
        println!("1. Use equipped weapon.");
        println!("2. Fight barehanded.");
    }

    fn print_player_choices(&self) {
        println!("Choose your action...");
        for (choice_num, action) in self.player_actions.iter().enumerate() {
            match action {
                PlayerAction::FightMonster(val) => {
                  println!("\t{}. Fight Monster ({val} Combat Power)", choice_num+1);
                },
                PlayerAction::DrinkPotion(val) => {
                    let amt = self.player.get_potion_heal_amount(*val);
                    println!("\t{}. Drink Potion (+{amt} HP)", choice_num+1)
                },
                PlayerAction::EquipWeapon(val) => {
                    println!("\t{}. Equip Weapon ({val} Damage)", choice_num+1)
                },
                PlayerAction::EscapeRoom => {
                    println!("\t{}. Escape room", choice_num+1);
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

    /// Escaping the room can only be done before the player takes any action in the room.
    /// The escape cooldown is set to 2 so that in the next room when the escape status is checked,
    /// the counter will be decremented by one, which is still not finished cooling down.
    /// This prevents the player from escaping two rooms in a row.
    fn escape_room(&mut self) {
        println!("You fled the room...");
        self.escape_cooldown = 1;
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