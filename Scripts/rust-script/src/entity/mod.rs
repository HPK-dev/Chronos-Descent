mod api;
mod inheritance;
use godot::{classes::CharacterBody2D, prelude::*};
use rustc_hash::FxHashMap;

use crate::tag::{damage::Damage, effect::Effect};

#[derive(Debug)]
struct EntityStats {
    max_health: f64,
    health: f64,
    max_mana: f64,
    mana: f64,
    strength: f64,
    intelligence: f64,
    defense: f64,
    attack_speed: f64,
    movement_speed: f64,
    attack_range: f64,
}

impl Default for EntityStats {
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
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Entity {
    base: Base<CharacterBody2D>,
    base_stats: EntityStats,
    current_stats: EntityStats,
    effects: FxHashMap<String, Effect>,
    damage_queue: Vec<Damage>,
    is_alive: bool,
}

/// TODO
impl Entity {
    /// Clear damage queue and calculate taken damage
    fn handle_queued_damage(&mut self) {
        let damage_queue = std::mem::take(&mut self.damage_queue);

        for damage in damage_queue {
            let amount: f64 = todo!("Calculate damage");
            self.current_stats.health -= amount;
            self.update_stats();
        }
    }

    fn handle_effect(&mut self, effect: &Effect) {
        todo!("Handle effect")
    }

    /// Re-calculate the current stats
    fn update_stats(&mut self) {
        if self.current_stats.health <= 0.0 {
            self.is_alive = false;
            return;
        }
        todo!("Update stats")
    }
}
