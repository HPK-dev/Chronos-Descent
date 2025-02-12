use enumset::{EnumSet, EnumSetType};
use godot::obj::Gd;

use crate::entity::Entity;

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
pub struct Damage {
    pub kind: EnumSet<DamageTag>,
    pub amount: f64,
    pub source: Gd<Entity>,
}

impl Damage {
    pub fn new(kind: EnumSet<DamageTag>, amount: f64, source: Gd<Entity>) -> Self {
        Self {
            kind,
            amount,
            source,
        }
    }
}
