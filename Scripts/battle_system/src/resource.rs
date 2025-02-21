use crate::component::{Equipment1, Equipment2, Equipment3, Equipment4, StatsModifyEffect, Weapon};
use crate::{component::CurrentStats, define_mapping};
use bevy_ecs::prelude::*;
use godot::obj::InstanceId;
use rustc_hash::FxHashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct EntitySnapshot {
    pub stats: CurrentStats,
    pub weapon: Weapon,
    pub eq1: Equipment1,
    pub eq2: Equipment2,
    pub eq3: Equipment3,
    pub eq4: Equipment4,
    pub effects: Vec<StatsModifyEffect>,
}

define_mapping! {
    #[derive(Resource)]
    GodotTimeScale => (f32);

    #[derive(Resource, Default)]
    GodotTimeDelta=>(f32);

    #[derive(Resource, Default)]
    GodotInstanceIdMap => (FxHashMap<InstanceId, Entity>);

    #[derive(Resource, Default)]
    EntitySnapshotMap => (FxHashMap<Uuid, (EntitySnapshot, usize)>);
}

impl Default for GodotTimeScale {
    fn default() -> Self {
        Self(1.0)
    }
}
