mod api;
mod inheritance;

use godot::{classes::CharacterBody2D, prelude::*};
use rustc_hash::FxHashMap;

use crate::tag::{damage::Damage, effect::Effect};

#[derive(Debug)]
pub struct EntityBaseStats {
    health: f64,
    mana: f64,
    strength: f64,
    intelligence: f64,
    defense: f64,
    attack_speed: f64,
    movement_speed: f64,
    attack_range: f64,
    crit_chance: f64,
    crit_damage: f64,
}

impl Default for EntityBaseStats {
    fn default() -> Self {
        Self {
            health: 100.0,
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

#[derive(Debug)]
pub struct EntityStats {
    max_health: f64,     // points
    health: f64,         // points
    max_mana: f64,       // points
    mana: f64,           // points
    strength: f64,       // points
    intelligence: f64,   // points
    defense: f64,        // points
    attack_speed: f64,   // times/second
    movement_speed: f64, // units/second
    attack_range: f64,   // units
    crit_chance: f64,    // percentage
    crit_damage: f64,    // percentage

    // Multipliers
    additive_multiplier: f64,       // percentage
    multiplicative_multiplier: f64, // percentage
    bouns_multiplier: f64,          // percentage
}

impl From<EntityBaseStats> for EntityStats {
    fn from(
        EntityBaseStats {
            health,
            mana,
            strength,
            intelligence,
            defense,
            attack_speed,
            movement_speed,
            attack_range,
            crit_chance,
            crit_damage,
        }: EntityBaseStats,
    ) -> Self {
        Self {
            max_health: health,
            health,
            max_mana: mana,
            mana,
            strength,
            intelligence,
            defense,
            attack_speed,
            movement_speed,
            attack_range,
            crit_chance,
            crit_damage,

            additive_multiplier: 0.0,
            multiplicative_multiplier: 1.0,
            bouns_multiplier: 0.0,
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Entity {
    base: Base<CharacterBody2D>,
    // Stats
    base_stats: EntityBaseStats,
    current_stats: EntityStats,
    // Effects
    effects: FxHashMap<String, Effect>,
    effect_timers: FxHashMap<String, f64>,
    // Damage queue
    damage_queue: Vec<Damage>,
    // Equipment slots
    // TODO
    // Inventory
    // TODO
    // Flags
    is_alive: bool,
    // TODO: effect flags
}

impl Entity {
    /// Clear damage queue and calculate taken damage
    fn handle_queued_damage(&mut self) {
        let damage_queue = if self.damage_queue.is_empty() {
            return;
        } else {
            std::mem::take(&mut self.damage_queue)
        };

        for damage in damage_queue {
            let amount: f64 = todo!("Calculate damage");
            self.current_stats.health -= amount;
            self.update_stats();
        }
    }

    /// Re-calculate the current stats
    fn update_stats(&mut self) {
        if self.current_stats.health <= 0.0 {
            self.is_alive = false;
            return;
        }
        todo!("Update stats")
    }

    fn take_snapshot(&self) -> EntityStats {
        todo!("Take a snapshot of the entity's stats")
    }

    fn update_effect_timers(&mut self, delta: f64) {
        self.effect_timers.iter_mut().for_each(|(_, timer)| {
            *timer -= delta;
        });

        let expired_effects: Vec<String> = self
            .effect_timers
            .iter()
            .filter_map(|(uuid, timer)| {
                if *timer <= 0.0 {
                    Some(uuid.clone())
                } else {
                    None
                }
            })
            .collect();

        expired_effects.into_iter().for_each(|uuid| {
            self.effects.remove(&uuid);
            self.effect_timers.remove(&uuid);
        });

        self.update_stats();
    }
}
