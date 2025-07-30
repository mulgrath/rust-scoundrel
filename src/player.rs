use std::cmp;

pub struct PlayerState {
    health: i32,
    max_health: i32,
    attack: i32,
    defense: i32,
    potion_on_cooldown: bool,
}

impl PlayerState {
    pub fn new(max_health: i32) -> PlayerState {
        PlayerState {health: max_health, max_health, attack: 0, defense: 0, potion_on_cooldown: false}
    }

    pub fn print_player(&self) {
        println!("Health: {}, Attack: {}, Defense: {}", self.health, self.attack, self.defense);
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn heal(&mut self, amount: i32) {
        self.potion_on_cooldown = true;
        self.health += amount;
        println!("Healed for {}. Health: {}/{}", amount, self.health, self.max_health);
    }

    pub fn remove_potion_cooldown(&mut self) {
        self.potion_on_cooldown = false;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        println!("Took {} damage. Health: {}/{}", amount, self.health, self.max_health);
    }

    pub fn equip(&mut self, val: i32) {
        println!("You equipped a weapon with {} damage.", val);
        self.attack = val;
    }

    pub fn get_potion_heal_amount(&self, val: &i32) -> i32 {
        // Potions can only be used once per room. Any further potions are simply discarded.
        if self.potion_on_cooldown {
            0
        }
        else {
            let amt = *val;
            let mut restore_amt = cmp::min(20 - (amt + self.health), amt);
            if restore_amt <=  0 {
                restore_amt = amt + restore_amt;
            }
            restore_amt
        }
    }
}

pub enum PlayerAction {
    EscapeRoom,
    EquipWeapon(i32),
    DrinkPotion(i32),
    FightMonster(i32),
}