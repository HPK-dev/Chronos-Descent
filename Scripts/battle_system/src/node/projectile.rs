use super::entity::Entity;
use crate::component::{Damage, DamageSource, DamageTag};
use crate::event::TakeDamageEvent;
use crate::resource::EntitySnapshotMap;
use crate::BattleSystem;
use enumset::{EnumSet, EnumSetType};
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

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

impl Default for ProjectileTarget {
    fn default() -> Self {
        Self::Velocity(Vector2::default())
    }
}
#[derive(GodotClass)]
#[class(no_init, base=CharacterBody2D)]
pub struct Projectile {
    pub kind: EnumSet<ProjectileTag>,
    pub damage: Option<Damage>,
    pub target: ProjectileTarget,
    pub base: Base<CharacterBody2D>,
}

pub struct ProjectileBuilder {
    projectile_kind: EnumSet<ProjectileTag>,
    target: ProjectileTarget,
    damage_kind: EnumSet<DamageTag>,
    base_damage: Option<f64>,
    snapshot: bool,
    amount: usize,
}

impl Default for ProjectileBuilder {
    fn default() -> Self {
        Self {
            projectile_kind: EnumSet::new(),
            target: ProjectileTarget::default(),
            damage_kind: EnumSet::new(),
            base_damage: None,
            snapshot: false,
            amount: 1,
        }
    }
}

impl ProjectileBuilder {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn set_amount(mut self, amount: usize) -> Self {
        self.amount = amount;
        self
    }

    pub fn add_damage_tag(mut self, value: impl Into<DamageTag>) -> Self {
        self.damage_kind.insert(value.into());
        self
    }

    pub fn set_damage(mut self, value: f64) -> Self {
        self.base_damage = Some(value);
        self
    }

    pub fn set_snapshot(mut self, snapshot: bool) -> Self {
        self.snapshot = snapshot;
        self
    }

    fn set_target(mut self, target: ProjectileTarget) -> Self {
        self.target = target;
        self
    }

    pub fn target_fixed_speed(self, target: Gd<Entity>, speed: f64) -> Self {
        self.set_target(ProjectileTarget::EntityFixedSpeed { target, speed })
    }

    pub fn target_fixed_time(self, target: Gd<Entity>, time: f64) -> Self {
        self.set_target(ProjectileTarget::EntityFixedTime { target, time })
    }

    fn calculate_velocity(
        from: Vector2,
        to: Vector2,
        speed_or_time: f32,
        is_speed: bool,
    ) -> Vector2 {
        if is_speed {
            let direction = from.try_direction_to(to).unwrap_or_default();
            direction * speed_or_time
        } else {
            (to - from) / speed_or_time
        }
    }

    pub fn point_fixed_speed(self, from: Vector2, to: Vector2, speed: f32) -> Self {
        let velocity = Self::calculate_velocity(from, to, speed, true);
        self.set_target(ProjectileTarget::Velocity(velocity))
    }

    pub fn point_fixed_time(self, from: Vector2, to: Vector2, time: f32) -> Self {
        let velocity = Self::calculate_velocity(from, to, time, false);
        self.set_target(ProjectileTarget::Velocity(velocity))
    }

    pub fn build(self, shooter: Gd<Entity>) -> Option<Gd<Projectile>> {
        let shooter_instance_id = shooter.instance_id();

        // Create damage source
        let damage_source = if self.snapshot {
            let mut battle_system =
                shooter.get_node_as::<BattleSystem>("/root/Autoload/BattleSystem");
            let mut battle_system = battle_system.bind_mut();
            battle_system
                .new_snapshot(&shooter_instance_id, self.amount)
                .map(DamageSource::Snapshot)
        } else {
            Some(DamageSource::Realtime(shooter_instance_id))
        };

        // Create the projectile if we have a valid damage source
        damage_source.map(|source| {
            let damage = self.base_damage.map(|base_amount| Damage {
                kind: self.damage_kind,
                base_amount,
                source,
            });

            Gd::from_init_fn(|base| Projectile {
                kind: self.projectile_kind,
                damage,
                target: self.target,
                base,
            })
        })
    }
}

#[godot_api]
impl ICharacterBody2D for Projectile {
    fn physics_process(&mut self, delta: f64) {
        let current = self.base().get_global_position();

        // Update projectile's position
        let velocity: Vector2 = match &mut self.target {
            ProjectileTarget::EntityFixedSpeed { target, speed } => {
                let target = target.get_global_position();
                let direction = current.direction_to(target);
                direction * *speed as f32 * delta as f32
            }
            ProjectileTarget::EntityFixedTime { target, time } => {
                *time -= delta;

                if *time <= 0.0 {
                    self.queue_free();
                    return;
                }

                let target = target.get_global_position();
                let distance = target - current;
                distance / *time as f32 * delta as f32
            }
            ProjectileTarget::Velocity(velocity) => *velocity * delta as f32,
        };

        // Detect collision
        let collision = self
            .base_mut()
            .move_and_collide(velocity)
            .and_then(|collision| collision.get_collider());
        if let Some(obj) = collision {
            self.handle_collision(obj)
        }
    }
}

impl Projectile {
    fn handle_collision(&mut self, body: Gd<Object>) {
        if let Ok(hit_entity) = body.try_cast::<Entity>() {
            if let Some(damage) = self.damage.take() {
                let event = TakeDamageEvent(hit_entity.instance_id(), damage);

                let base = self.base();
                let mut battle_system =
                    base.get_node_as::<BattleSystem>("/root/Autoload/BattleSystem");
                let mut battle_system = battle_system.bind_mut();

                battle_system.world.trigger(event);
            }
        }
        self.queue_free();
    }

    fn queue_free(&mut self) {
        // Check is need to update snapshot ref counter
        if let Some(damage) = &self.damage {
            if let DamageSource::Snapshot(id) = damage.source {
                let base = self.base();
                let mut battle_system =
                    base.get_node_as::<BattleSystem>("/root/Autoload/BattleSystem");
                let mut battle_system = battle_system.bind_mut();
                let mut snapshot_map = battle_system
                    .world
                    .get_resource_mut::<EntitySnapshotMap>()
                    .unwrap();

                if let Some((_, ref_count)) = snapshot_map.get_mut(&id) {
                    *ref_count -= 1
                }
            }
        }

        self.base_mut().queue_free();
    }

    fn mount_image(&mut self, res_path: impl AsRef<String>) {
        unimplemented!(
            "Maybe we need this function to avoid create every scene for each projectile"
        )
    }
}

/* TODO:
   - Build projectiles direct in Rust code. (use a Enum to store it)
*/
