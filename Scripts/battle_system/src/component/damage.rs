use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use godot::obj::InstanceId;

use super::CurrentStats;

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum DamageTag {
    // == Elements ==
    // Fire,
    // Water,
    // Ice,
    // Thunder,

    // == Damage Types ==
    Physical,
    Magic,
    Real,
    Skill,
    Projectile,
}

pub enum DamageSource {
    Realtime(InstanceId),
    Snapshot(CurrentStats),
}

pub struct Damage {
    pub kind: EnumSet<DamageTag>,
    pub base_amount: f64,
    pub source: DamageSource,
}

impl Damage {
    pub fn new(kind: EnumSet<DamageTag>, amount: f64, source: DamageSource) -> Self {
        Self {
            kind,
            base_amount: amount,
            source,
        }
    }
}

#[derive(Component)]
pub struct DamageQueue(pub Vec<Damage>);
