use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct InstanceId(pub godot::obj::InstanceId);

impl PartialEq<godot::obj::InstanceId> for InstanceId {
    fn eq(&self, other: &godot::obj::InstanceId) -> bool {
        self.0 == *other
    }
}

impl PartialEq<InstanceId> for godot::obj::InstanceId {
    fn eq(&self, other: &InstanceId) -> bool {
        *self == other.0
    }
}
