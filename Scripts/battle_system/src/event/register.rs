use crate::bundle::EntityBundle;
use crate::resource::GodotInstanceIdMap;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::observer::Trigger;
use bevy_ecs::prelude::{Commands, Event};
use godot::global::godot_print;
use godot::obj::InstanceId as GodotInstanceId;

#[derive(Event)]
pub struct RegisterEntityEvent(pub GodotInstanceId);

#[derive(Event)]
pub struct UnregisterEntityEvent(pub GodotInstanceId);

pub fn register_entity(
    trigger: Trigger<RegisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {

    let godot_instance_id = trigger.event().0;
    let entity = cmd.spawn(EntityBundle::from(godot_instance_id)).id();
    index.insert(godot_instance_id, entity);

    #[cfg(debug_assertions)]
    godot_print!("Register entity: {:?}", godot_instance_id);
}

pub fn unregister_entity(
    trigger: Trigger<UnregisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {
    let godot_instance_id = trigger.event().0;
    if let Some(entity) = index.remove(&godot_instance_id) {
        cmd.entity(entity).despawn();
    }

    godot_print!("Unregister entity: {:?}", godot_instance_id);
}
