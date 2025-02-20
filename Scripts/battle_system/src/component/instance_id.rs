use bevy_ecs::prelude::*;
use godot::obj::InstanceId;

use crate::define_mapping;

define_mapping! {
    #[derive(Clone, Component)]
    GodotInstanceId => (InstanceId);
}

impl Default for GodotInstanceId {
    fn default() -> Self {
        Self(InstanceId::from_i64(i64::MIN))
    }
}
