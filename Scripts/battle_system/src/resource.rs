use bevy_ecs::prelude::*;
use godot::obj::InstanceId;
use rustc_hash::FxHashMap;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Resource, Default)]
pub struct GodotTimeDelta(pub f64);

#[derive(Resource)]
pub struct GodotTimeScale(pub f64);

impl Default for GodotTimeScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource, Default)]
pub struct GodotInstanceIdMap(pub FxHashMap<InstanceId, Entity>);

impl Deref for GodotInstanceIdMap {
    type Target = FxHashMap<InstanceId, Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GodotInstanceIdMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
