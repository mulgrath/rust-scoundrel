use std::cmp;

pub struct PlayerState {
    health: i32,
    max_health: i32,
    weapon_power: i32,
    weapon_durability: i32,
    potion_on_cooldown: bool,
}

impl PlayerState {
    pub fn new(max_health: i32) -> PlayerState {
        PlayerState {health: max_health, max_health, weapon_power: 0, weapon_durability: 0, potion_on_cooldown: false}
    }

    pub fn print_player(&self) {
        println!("== PLAYER ==");
        if self.weapon_durability == i32::MAX {
            println!("Health: {}, Weapon Power: {}, Weapon Durability: MAX", self.health, self.weapon_power);
        }
        else {
            println!("Health: {}, Weapon Power: {}, Weapon Durability: {}", self.health, self.weapon_power, self.weapon_durability);
        }
        println!("============")
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

    /// Equips the selected weapon, where the card's value is its power.
    /// A newly equipped weapon has functionally infinite durability as it can be used to attack
    /// any monster, regardless of its power.
    pub fn equip(&mut self, val: i32) {
        println!("You equipped a weapon with {} damage.", val);
        self.weapon_power = val;
        self.weapon_durability = i32::MAX;
    }

    pub fn can_use_weapon(&mut self, monster_power: i32) -> bool {
        self.weapon_durability > monster_power
    }

    pub fn attack_monster(&mut self, monster_power: i32, use_weapon: bool) {
        if use_weapon {
            let damage_to_take = cmp::max(monster_power - self.weapon_power, 0);
            self.take_damage(damage_to_take);
            self.weapon_durability = monster_power;
        }
        else {
            self.take_damage(monster_power);
        }
    }

    pub fn get_potion_heal_amount(&self, val: i32) -> i32 {
        // Potions can only be used once per room. Any further potions are useless.
        if self.potion_on_cooldown {
            0
        }
        else if val + self.health > self.max_health {
            self.max_health - self.health
        }
        else {
            val
        }
    }
}

pub enum PlayerAction {
    EscapeRoom,
    EquipWeapon(i32),
    DrinkPotion(i32),
    FightMonster(i32),
}