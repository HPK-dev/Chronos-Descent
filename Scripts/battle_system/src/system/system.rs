use bevy_ecs::prelude::*;

use crate::{
    component::EffectsTimer,
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
