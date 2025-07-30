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
        println!("Health: {}, Attack: {}, Defense: {}", self.health, self.weapon_power, self.weapon_durability);
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

    pub fn attack_monster(&mut self, monster_power: i32) {
        let mut barehanded = false;
        let mut can_use_weapon = false;

        // Determine if current weapon is usable
        if self.weapon_durability > monster_power {
            can_use_weapon = true;
        }
        else {
            barehanded = true;
        }

        // Decide to fight barehanded or use a weapon (Weapon can only be used if its durability is higher than the monster's power
        if can_use_weapon {
            // TODO: This should be a choice. For now, always use the weapon if we can.
            barehanded = false;
        }

        // Fight using hands or weapon
        if barehanded {
            self.take_damage(monster_power);
        }
        else {
            let damage_to_take = cmp::max(monster_power - self.weapon_power, 0);
            self.take_damage(damage_to_take);
            self.weapon_durability = monster_power;
        }
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