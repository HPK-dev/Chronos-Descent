use crate::component::{
    CrowdControlEffects, Effect, EffectMetadata, EffectsQueue, EffectsTimer, GetEffectId,
    ModifierEffects, TickEffects,
};
use bevy_ecs::change_detection::Res;
use bevy_ecs::event::Event;
use bevy_ecs::observer::Trigger;
use bevy_ecs::prelude::ResMut;
use bevy_ecs::system::Query;
use godot::global::godot_print;
use uuid::Uuid;

use crate::resource::GodotInstanceIdMap;
use godot::prelude::InstanceId as GodotInstanceId;

#[derive(Event)]
pub struct RemoveEffectEvent(pub GodotInstanceId, pub Uuid);

#[derive(Event)]
pub struct ApplyEffectEvent(pub GodotInstanceId, pub Effect);

fn create_effects_map<T>(
    owner: &GodotInstanceId,
    effects: &mut Vec<(T, f32)>,
    runtime_uuid: Uuid,
    timer: &mut ResMut<EffectsTimer>,
) -> Vec<T>
where
    T: Clone + GetEffectId,
{
    let effects = std::mem::take(effects);

    effects
        .into_iter()
        .map(|(effect, duration)| {
            timer.insert(runtime_uuid, (duration, *owner));
            effect
        })
        .collect()
}

pub fn apply_effect(
    mut trigger: Trigger<ApplyEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(
        &mut EffectsQueue,
        &mut TickEffects,
        &mut ModifierEffects,
        &mut CrowdControlEffects,
    )>,
    mut timer: ResMut<EffectsTimer>,
) {
    #[cfg(debug_assertions)]
    godot_print!("apply_effect");

    let instance_id = &trigger.0.clone();

    let (mut player_effect_queue, mut tick_effects, mut modifier_effects, mut cc_effects) =
        match index.get(&trigger.0).map(|entity| query.get_mut(*entity)) {
            Some(Ok(entity)) => entity,
            _ => return,
        };

    let Effect {
        id,
        visible,
        modifier,
        cc,
        tick,
    } = &mut trigger.event_mut().1;
    let runtime_uuid = Uuid::new_v4();

    let tick_efs = create_effects_map(instance_id, tick, runtime_uuid, &mut timer);
    let cc_efs = create_effects_map(instance_id, cc, runtime_uuid, &mut timer);
    let modifier_efs = create_effects_map(instance_id, modifier, runtime_uuid, &mut timer);

    let metadata = EffectMetadata {
        id: id.to_string(),
        visible: *visible,
    };

    tick_effects.insert(runtime_uuid, tick_efs);
    cc_effects.insert(runtime_uuid, cc_efs);
    modifier_effects.insert(runtime_uuid, modifier_efs);
    player_effect_queue.insert(runtime_uuid, metadata);
}

pub fn remove_effect(
    trigger: Trigger<RemoveEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(
        &mut EffectsQueue,
        &mut TickEffects,
        &mut ModifierEffects,
        &mut CrowdControlEffects,
    )>,
    mut timer: ResMut<EffectsTimer>,
) {
    #[cfg(debug_assertions)]
    godot_print!("remove_effect");

    let (mut player_effect_queue, mut tick_effects, mut modifier_effects, mut cc_effects) =
        match index.get(&trigger.0).map(|entity| query.get_mut(*entity)) {
            Some(Ok(entity)) => entity,
            _ => return,
        };

    let runtime_uuid = trigger.1;
    player_effect_queue.remove(&runtime_uuid);

    tick_effects.remove(&runtime_uuid);
    cc_effects.remove(&runtime_uuid);
    modifier_effects.remove(&runtime_uuid);

    timer.retain(|id, _| *id != runtime_uuid)
}
