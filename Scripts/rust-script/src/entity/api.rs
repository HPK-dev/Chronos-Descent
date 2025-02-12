use enumset::EnumSet;
use godot::prelude::*;

use crate::tag::{
    damage::{Damage, DamageTag},
    effect::{Effect, EffectDuration, EffectTag},
};

use super::Entity;

#[godot_api]
impl Entity {
    #[func]
    fn melee_attack(&mut self, target: Gd<Entity>) {
        self.melee_attack_internal(target)
    }

    #[func]
    fn ranged_attack(&mut self, target: Gd<Entity>) {
        self.ranged_attack_internal(target)
    }

    #[func]
    fn take_damage(&mut self, kind: Array<GString>, amount: f64, source: Gd<Entity>) {
        let kind: EnumSet<DamageTag> = kind
            .iter_shared()
            .map(|tag| tag.to_string().parse::<DamageTag>().unwrap())
            .collect();

        self.take_damage_internal(Damage::new(kind, amount, source))
    }

    #[func]
    fn add_effect(
        &mut self,
        kind: GString,
        amount: f64,
        duration: f64,
        damage_tags: Array<GString>,
    ) {
        let damage_tags: EnumSet<DamageTag> = damage_tags
            .iter_shared()
            .map(|tag| tag.to_string().parse::<DamageTag>().unwrap())
            .collect();
        let duration = duration.into();
        let kind = kind.to_string().parse::<EffectTag>().unwrap();

        self.add_effect_internal(Effect::new(kind, amount, duration, damage_tags))
    }

    #[func]
    fn remove_effect(&mut self, uuid: GString) {
        self.remove_effect_internal(uuid.to_string())
    }
}

impl Entity {
    pub fn melee_attack_internal(&mut self, target: Gd<Entity>) {
        let can_be_reached: bool = todo!("Check target can be reached");
        if !can_be_reached {
            return;
        }
        todo!("Call `target.take_damage` to dealt damage")
    }

    pub fn ranged_attack_internal(&mut self, target: Gd<Entity>) {
        todo!("Spawn a projectile")
    }

    pub fn skill_attack_internal(&mut self, target: Gd<Entity>) {
        unimplemented!("Need a skill argument");
        todo!()
    }

    pub fn take_damage_internal(&mut self, damage: Damage) {
        self.damage_queue.push(damage);
    }

    pub fn add_effect_internal(&mut self, effect: Effect) {
        self.handle_effect(&effect);

        let uuid = uuid::Uuid::new_v4().to_string();
        match &effect.duration {
            EffectDuration::Permanent => {
                self.effects.insert(uuid, effect);
            }
            EffectDuration::Temporary(duration) => {
                self.effects.insert(uuid, effect);
                todo!("Setup timer");
            }
            EffectDuration::Instant => {
                return;
            }
        }
    }

    pub fn remove_effect_internal(&mut self, uuid: String) {
        self.effects.remove(&uuid);
        self.update_stats();
    }
}
