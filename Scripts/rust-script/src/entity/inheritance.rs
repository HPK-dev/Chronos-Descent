use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;
use rustc_hash::FxHashMap;

use crate::entity::EntityStats;

use super::Entity;

const INITIAL_DAMAGE_QUEUE_CAPACITY: usize = 16;

/// Inherited methods from the `CharacterBody2D` class
#[godot_api]
impl ICharacterBody2D for Entity {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Entity: Initializing");

        Self {
            base,
            base_stats: EntityStats::default(),
            current_stats: EntityStats::default(),
            effects: FxHashMap::default(),
            effect_timers: FxHashMap::default(),
            damage_queue: Vec::with_capacity(INITIAL_DAMAGE_QUEUE_CAPACITY),
            is_alive: true,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if !self.is_alive {
            let mut this = self.base_mut();
            this.queue_free();
            return;
        }

        self.update_effect_timers(delta);

        todo!("Handle queued incoming damage");
    }
}
