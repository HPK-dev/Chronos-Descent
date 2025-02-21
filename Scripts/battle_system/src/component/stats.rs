use bevy_ecs::prelude::*;
use std::fmt::Display;

// TODO: Maybe fetch base stats with some formula?

#[derive(Component, Clone)]
pub struct CurrentStats {
    pub max_health: f64,     // points
    pub health: f64,         // points
    pub max_mana: f64,       // points
    pub mana: f64,           // points
    pub strength: f64,       // points
    pub intelligence: f64,   // points
    pub defense: f64,        // points
    pub attack_speed: f64,   // times/second
    pub movement_speed: f64, // units/second
    pub attack_range: f64,   // units
    pub crit_chance: f64,    // percentage
    pub crit_damage: f64,    // percentage
}

impl Default for CurrentStats {
    fn default() -> Self {
        Self {
            max_health: 100.0,
            health: 100.0,
            max_mana: 100.0,
            mana: 100.0,
            strength: 10.0,
            intelligence: 10.0,
            defense: 10.0,
            attack_speed: 1.0,
            movement_speed: 1.0,
            attack_range: 4.0,
            crit_chance: 50.0,
            crit_damage: 100.0,
        }
    }
}

impl Display for CurrentStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Stats
        ======================
        Health: {}/{}
        Mana: {}/{}
        Strength: {}
        Intelligence: {}
        Defense: {}
        Movement speed: {}
        Attack speed: {}
        Attack range: {}
        Crit. Chance: {}%
        Crit. Damage: {}%
        "#,
            self.health,
            self.max_health,
            self.mana,
            self.max_mana,
            self.strength,
            self.intelligence,
            self.defense,
            self.movement_speed,
            self.attack_speed,
            self.attack_range,
            self.crit_chance,
            self.crit_damage
        )
    }
}
