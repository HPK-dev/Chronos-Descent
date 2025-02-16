use bevy_ecs::prelude::*;
use godot::global::godot_print;

use crate::{
    component::{BaseStats, CurrentStats, Effects, EffectsTimer},
    resource::{GodotTimeDelta, GodotTimeScale},
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

#[allow(unused_variables)]
pub fn effects_changed_update(
    mut query: Query<(&BaseStats, &Effects, &mut CurrentStats), Changed<Effects>>,
) {
    for (base, effects, mut currents) in query.iter() {
        // TODO: Implement this
        godot_print!("Stats update!")
    }
}

pub fn current_stats_update(query: Query<Entity, Changed<CurrentStats>>) {}
