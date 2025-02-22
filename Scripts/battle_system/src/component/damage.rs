use bevy_ecs::component::Component;
use enumset::{EnumSet, EnumSetType};
use godot::obj::InstanceId;
use uuid::Uuid;
use crate::define_mapping;

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum DamageTag {
    // == Damage Types ==
    Physical,
    Magic,
    Real,
    Skill,
    Projectile,
}

#[derive(Clone)]
pub enum DamageSource {
    Realtime(InstanceId),
    Snapshot(Uuid),
}

#[derive(Clone)]
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

define_mapping! {
    #[derive(Component, Default)]
    DamageQueue => (Vec<Damage>);
}

