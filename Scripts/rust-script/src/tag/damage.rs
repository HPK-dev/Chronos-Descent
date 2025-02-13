use enumset::{EnumSet, EnumSetType};
use godot::obj::Gd;

use crate::entity::{Entity, EntityStats};

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

#[derive(Debug)]
pub enum DamageSource {
    Realtime(Gd<Entity>),
    Snapshot(EntityStats),
}

#[derive(Debug)]
pub struct Damage {
    pub kind: EnumSet<DamageTag>,
    pub amount: f64,
    pub source: DamageSource,
}

impl Damage {
    pub fn new(kind: EnumSet<DamageTag>, amount: f64, source: DamageSource) -> Self {
        Self {
            kind,
            amount,
            source,
        }
    }
}
