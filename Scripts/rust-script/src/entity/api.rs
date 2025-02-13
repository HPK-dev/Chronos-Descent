use enumset::EnumSet;
use godot::prelude::*;

use crate::{
    projectile::Projectile,
    tag::{
        damage::{Damage, DamageSource, DamageTag},
        effect::{Effect, EffectDuration, EffectTag},
    },
};

use super::Entity;

#[godot_api]
impl Entity {
    #[func]
    fn spawn_projectile(&mut self, projectile: Gd<Projectile>) {
        self.__spawn_projectile(projectile)
    }

    #[func]
    fn take_damage(
        &mut self,
        kind: Array<GString>,
        amount: f64,
        source: Gd<Entity>,
        snapshot: bool,
    ) {
        let kind: EnumSet<DamageTag> = kind
            .iter_shared()
            .map(|tag| tag.to_string().parse::<DamageTag>().unwrap())
            .collect();

        let source = if snapshot {
            DamageSource::Snapshot(source.bind().take_snapshot())
        } else {
            DamageSource::Realtime(source)
        };

        self.__take_damage(Damage::new(kind, amount, source))
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

        self.__add_effect(Effect::new(kind, amount, duration, damage_tags))
    }

    #[func]
    fn remove_effect(&mut self, uuid: GString) {
        self.__remove_effect(uuid.to_string())
    }
}

impl Entity {
    pub fn __spawn_projectile(&mut self, projectile: Gd<Projectile>) {
        todo!("Spawn a projectile")
    }

    pub fn __cast_skill(&mut self) {
        unimplemented!("Need a skill argument");
    }

    pub fn __take_damage(&mut self, damage: Damage) {
        self.damage_queue.push(damage);
    }

    pub fn __add_effect(&mut self, effect: Effect) {
        let uuid = uuid::Uuid::new_v4().to_string();
        match &effect.duration {
            EffectDuration::Permanent => {
                self.effects.insert(uuid, effect);
            }
            EffectDuration::Temporary(duration) => {
                self.effect_timers.insert(uuid.clone(), *duration);
                self.effects.insert(uuid, effect);
            }
            EffectDuration::Instant => {
                return;
            }
        }

        self.update_stats();
    }

    pub fn __remove_effect(&mut self, uuid: String) {
        self.effects.remove(&uuid);
        self.effect_timers.remove(&uuid);
        self.update_stats();
    }
}
