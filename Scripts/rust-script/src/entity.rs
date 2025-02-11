use std::collections::{HashMap, HashSet};

use godot::{
    classes::{CharacterBody2D, ICharacterBody2D},
    obj::WithBaseField,
    prelude::*,
};

use crate::tag::{
    damage::{Damage, DamageTag},
    effect::{Effect, EffectDuration, EffectTag},
};

const INITIAL_EFFECTS_CAPACITY: usize = 16;
const INITIAL_MULTIPLIER_CAPACITY: usize = 8;
const INITIAL_DAMAGE_QUEUE_CAPACITY: usize = 16;

// Stats struct to group related fields
#[derive(Debug, Clone)]
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
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Entity {
    base: Base<CharacterBody2D>,
    base_stats: EntityStats,
    current_stats: EntityStats,
    effects: Vec<Effect>,
    damage_queue: Vec<Damage>,
    is_alive: bool,
}

#[derive(Debug, Default)]
struct EntityDamageFactors {
    pub additive: Vec<f64>,
    pub multiplicative: Vec<f64>,
}

impl EntityDamageFactors {
    fn new() -> Self {
        Self {
            additive: Vec::with_capacity(INITIAL_MULTIPLIER_CAPACITY),
            multiplicative: Vec::with_capacity(INITIAL_MULTIPLIER_CAPACITY),
        }
    }

    fn calculate_total(&self) -> (f64, f64) {
        (
            self.additive.iter().sum(),
            self.multiplicative.iter().product(),
        )
    }
}

#[godot_api]
impl ICharacterBody2D for Entity {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Entity: Initializing");

        Self {
            base,
            base_stats: EntityStats::default(),
            current_stats: EntityStats::default(),
            effects: Vec::with_capacity(INITIAL_EFFECTS_CAPACITY),
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

        self.handle_effects(delta);
        self.handle_damage();
    }
}

impl Entity {
    fn take_stats_snapshot(&self) -> EntityStats {
        self.current_stats.clone()
    }

    fn handle_damage(&mut self) {
        let damage_queue = std::mem::take(&mut self.damage_queue);

        if damage_queue.is_empty() {
            return;
        }

        let damage_factor = self.calculate_damage_factors();
        let mut source_cache: HashMap<InstanceId, EntityStats> = HashMap::new();

        for damage in damage_queue {
            self.apply_damage(
                &damage.kind,
                damage.amount,
                &damage.source,
                &damage_factor,
                &mut source_cache,
            );
        }
    }

    fn handle_effects(&mut self, delta: f64) {
        let current_effects = std::mem::take(&mut self.effects);

        self.effects.clear();
        self.effects.reserve(current_effects.len());

        for effect in current_effects {
            self.apply_effect(&effect.kind, effect.amount, &effect.source);

            match effect.duration {
                EffectDuration::Permanent => self.effects.push(effect),
                EffectDuration::Temporary(duration) if duration > delta => {
                    self.effects.push(Effect {
                        duration: EffectDuration::Temporary(duration - delta),
                        ..effect
                    });
                }
                _ => {}
            }
        }
    }
}

// TODO
impl Entity {
    /// Iterates over all effects, armors, weapon, talents, etc.
    /// to calculate the final damage factors
    fn calculate_damage_factors(&self) -> EntityDamageFactors {
        let mut factors = EntityDamageFactors::new();
        // TODO: Implement damage factors calculation
        factors
    }

    fn apply_effect(&mut self, kind: &EffectTag, amount: f64, source: &Gd<Entity>) {
        // TODO: Implement effect logic
        godot_print!("Entity: Applied effect: {kind}, {amount}, {source}");
    }

    fn apply_damage(
        &mut self,
        kind: &HashSet<DamageTag>,
        amount: f64,
        source: &Gd<Entity>,
        factor: &EntityDamageFactors,
        source_cache: &mut HashMap<InstanceId, EntityStats>,
    ) {
        // TODO: Implement damage logic

        godot_print!(
            "Entity: Applied damage: {:#?}, {}, {}",
            kind,
            amount,
            source
        );
    }
}

#[godot_api]
impl Entity {
    #[func]
    fn take_damage(&mut self, tags: Array<GString>, amount: f64, source: i64) {
        let kind = HashSet::from_iter(
            tags.iter_shared()
                .map(|tag| tag.to_string().parse().unwrap()),
        );

        self.damage_queue.push(Damage {
            kind,
            amount,
            source: Gd::from_instance_id(InstanceId::from_i64(source)),
        });
    }

    #[func]
    fn add_effect(&mut self, kind: String, amount: f64, duration: f64, source: i64) {
        self.effects.push(Effect {
            kind: kind.parse().expect(&format!("Unknown effect tag: {kind}")),
            amount,
            duration: EffectDuration::from(duration),
            source: Gd::from_instance_id(InstanceId::from_i64(source)),
        });
    }
}
