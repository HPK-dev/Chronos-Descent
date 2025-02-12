use godot::{classes::IArea2D, obj::WithBaseField, prelude::*};

use crate::projectile::Projectile;

use super::ProjectileTarget;

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
