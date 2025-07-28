pub struct PlayerState {
    health: i32,
    attack: i32,
    defense: i32,
}

impl PlayerState {
    pub fn new(health: i32, attack: i32, defense: i32) -> PlayerState {
        PlayerState {health, attack, defense}
    }

    pub fn print_player(&self) {
        println!("Health: {}, Attack: {}, Defense: {}", self.health, self.attack, self.defense);
    }

    pub fn health(&self) -> i32 {
        self.health
    }
}

pub enum PlayerAction {
    EscapeRoom,
    EquipWeapon(i32),
    DrinkPotion(i32),
    FightMonster(i32),
}