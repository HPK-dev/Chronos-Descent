use enumset::{EnumSet, EnumSetType};
use godot::classes::Area2D;
use godot::classes::IArea2D;
use godot::prelude::*;

use crate::component::Damage;

use super::entity::Entity;

#[derive(strum::EnumString, strum::Display, EnumSetType)]
pub enum ProjectileTag {
    Piercing,
    Bouncing,
}

pub enum ProjectileTarget {
    EntityFixedSpeed { target: Gd<Entity>, speed: f64 },
    EntityFixedTime { target: Gd<Entity>, time: f64 },
    Velocity(Vector2),
}

#[derive(GodotClass)]
#[class(no_init, base=Area2D)]
pub struct Projectile {
    pub kind: EnumSet<ProjectileTag>,
    pub damage: Damage,
    pub target: ProjectileTarget,
    pub base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Projectile {
    fn physics_process(&mut self, delta: f64) {
        let current = self.base().get_global_position();

        let velocity: Vector2 = match &mut self.target {
            ProjectileTarget::EntityFixedSpeed { target, speed } => {
                let target = target.get_global_position();
                let direction = target - current;
                direction.normalized() * *speed as f32 * delta as f32
            }
            ProjectileTarget::EntityFixedTime { target, time } => {
                let target = target.get_global_position();
                let direction = target - current;
                let v = direction.normalized() * direction.length() / *time as f32 * delta as f32;
                *time -= delta;
                v
            }
            ProjectileTarget::Velocity(velocity) => *velocity * delta as f32,
        };

        self.base_mut().translate(velocity);
    }
}
