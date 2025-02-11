use godot::obj::Gd;

use crate::entity::Entity;

#[derive(Debug, strum::EnumString, strum::Display)]
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
    Ability,
    Projectile,
}

#[derive(Debug)]
pub struct Damage {
    pub kind: DamageTag,
    pub amount: f64,

    pub source: Gd<Entity>,
}
