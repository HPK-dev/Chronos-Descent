use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;

use crate::tag::damage::Damage;

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Entity {
    base: Base<CharacterBody2D>,

    max_health: f64,
    health: f64,
    max_mana: f64,
    mana: f64,

    strength: f64,
    intelligence: f64,
    defense: f64,

    attack_speed: f64,
    movement_speed: f64,

    __damage_queue: Vec<Damage>,
    __is_alive: bool,
}

#[godot_api]
impl ICharacterBody2D for Entity {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Initalize an entity");

        Self {
            base,
            max_health: 100.0,
            health: 100.0,
            max_mana: 100.0,
            mana: 100.0,
            strength: 10.0,
            intelligence: 10.0,
            defense: 10.0,
            attack_speed: 1.0,
            movement_speed: 1.0,
            __damage_queue: Vec::new(),
            __is_alive: true,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        godot_print!("Entity physics process");
    }
}

/// Internal methods
impl Entity {
    fn take_queued_damage(&mut self) {
        todo!()
    }
}

#[godot_api]
impl Entity {
    #[func]
    fn take_damage(&mut self, kind: String, amount: f64, source: String) {
        let damage = Damage {
            kind: kind.parse().expect("Unknown damage kind"),
            amount,
            source,
        };
        self.__damage_queue.push(damage);
    }

    #[func]
    fn heal(&mut self, amount: f64) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
