use enumset::{EnumSet, EnumSetType};
use godot::{builtin::Vector2, obj::Gd};

use crate::entity::Entity;

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum ProjectileTag {
    Directional,
    Homing,
    Piercing,
    Bouncing,
}

#[derive(Debug)]
pub enum ProjectileTarget {
    Entity(Gd<Entity>),
    Position((f64, f64)),
    Velocity(Vector2),
}

#[derive(Debug)]
pub struct Projectile {
    pub kind: EnumSet<ProjectileTag>,
    pub damage: f64,
    pub source: Gd<Entity>,
    pub target: ProjectileTarget,
}
