use crate::component::{EffectsTimer, TickEffects};
use crate::event::RemoveEffectEvent;
use crate::resource::{GodotTimeDelta, GodotTimeScale};
use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::{Query, ResMut};
use bevy_ecs::system::Commands;

pub fn effect_timer_update(
    delta: Res<GodotTimeDelta>,
    scale: Res<GodotTimeScale>,
    mut effect_timers: ResMut<EffectsTimer>,
    mut cmd: Commands,
) {
    let time = delta.0 * scale.0;

    let mut expired_ids = Vec::new();

    effect_timers
        .iter_mut()
        .for_each(|(id, (duration, instance_id))| {
            if *duration - time <= 0.0 {
                expired_ids.push((id, instance_id));
            } else {
                *duration -= time;
            }
        });

    expired_ids
        .into_iter()
        .for_each(|(runtime_id, instance_id)| {
            cmd.trigger(RemoveEffectEvent(*instance_id, *runtime_id))
        })
}

pub fn tick_effect_update(
    delta: Res<GodotTimeDelta>,
    scale: Res<GodotTimeScale>,
    mut query: Query<&mut TickEffects>,
) {
    let time = delta.0 * scale.0;

    for mut effect in query.iter_mut() {
        for e in &mut effect.0.values_mut() {
            for effect in e {
                effect.__interval_counter += time;

                if effect.__interval_counter > effect.interval {
                    effect.__interval_counter = 0.0;

                    todo!("Trigger effect!")
                }
            }
        }
    }
}
