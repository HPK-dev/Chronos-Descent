mod inheritance;

use enumset::{EnumSet, EnumSetType};
use godot::{
    builtin::Vector2,
    classes::Area2D,
    obj::{Base, Gd},
    prelude::GodotClass,
};

use crate::entity::Entity;

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum ProjectileTag {
    Piercing,
    Bouncing,
}

#[derive(Debug)]
pub enum ProjectileTarget {
    EntityFixedSpeed { target: Gd<Entity>, speed: f64 },
    EntityFixedTime { target: Gd<Entity>, time: f64 },
    Velocity(Vector2),
}

#[derive(Debug, GodotClass)]
#[class(no_init, base=Area2D)]
pub struct Projectile {
    pub kind: EnumSet<ProjectileTag>,
    pub damage: f64,
    pub source: Gd<Entity>,
    pub target: ProjectileTarget,
    pub base: Base<Area2D>,
}
