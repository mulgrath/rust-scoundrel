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
}