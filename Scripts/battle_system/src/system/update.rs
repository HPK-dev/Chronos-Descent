use bevy_ecs::prelude::*;
use godot::global::godot_print;

use crate::{
    component::{BaseStats, CurrentStats, Effects, EffectsTimer},
    resource::{EntitySnapshotMap, GodotTimeDelta, GodotTimeScale},
};

pub fn effect_timer_update(
    delta: Res<GodotTimeDelta>,
    scale: Res<GodotTimeScale>,
    mut query: Query<&mut EffectsTimer>,
) {
    let mut expired = Vec::new();
    let period = delta.0 * scale.0;

    for mut timers in query.iter_mut() {
        for (id, t) in timers.0.iter_mut() {
            *t -= period;

            if *t <= 0.0 {
                expired.push(*id);
            }
        }
    }
}

pub fn effects_changed_update(
    mut query: Query<(&BaseStats, &Effects, &mut CurrentStats), Changed<Effects>>,
) {
    // #[cfg(debug_assertions)]
    // godot_print!("Stats update!");

    for (base, effects, mut currents) in query.iter() {
        // TODO: Implement this
    }
}

pub fn current_stats_update(query: Query<Entity, Changed<CurrentStats>>) {
    #[cfg(debug_assertions)]
    godot_print!("Current stats update!");

    todo!();
}

pub fn snapshot_ref_count_update(mut counter: ResMut<EntitySnapshotMap>, mut cmd: Commands) {
    if counter.is_changed() {
        #[cfg(debug_assertions)]
        godot_print!("Snapshot ref count update!");

        counter.retain(|_, (entity, count)| {
            if *count == 0 {
                cmd.entity(*entity).despawn();
                false
            } else {
                true
            }
        });
    }
}
