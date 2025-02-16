use bevy_ecs::prelude::*;
use godot::obj::InstanceId;
use std::ops::{Deref, DerefMut};

#[derive(Component)]
pub struct GodotInstanceId(pub InstanceId);

impl Deref for GodotInstanceId {
    type Target = InstanceId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GodotInstanceId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
