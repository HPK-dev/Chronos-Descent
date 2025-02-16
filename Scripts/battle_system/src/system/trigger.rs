use bevy_ecs::{
    observer::Trigger,
    system::{Commands, Query, Res, ResMut},
};
use enumset::EnumSet;
use uuid::Uuid;

use crate::{
    bundle::EntityBundle,
    component::{
        CurrentStats, Damage, DamageSource, DamageTag, EffectDuration, Effects, EffectsTimer,
    },
    event::{
        ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, RemoveEffectsEvent,
        TakeDamageEvent,
    },
    resource::GodotInstanceIdMap,
};

// ========================| TODO |========================

fn calculate_raw_damage(kind: &EnumSet<DamageTag>, base_amount: f64, stats: &CurrentStats) -> f64 {
    todo!()
}

fn calculate_damage_reduction(
    kind: &EnumSet<DamageTag>,
    base_amount: f64,
    stats: &CurrentStats,
) -> f64 {
    todo!()
}

// ========================================================

pub fn take_damage(
    trigger: Trigger<TakeDamageEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut CurrentStats)>,
) {
    let godot_instance_id = trigger.event().0;

    let Some(attackee_stats) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get(*entity).ok())
    else {
        return;
    };

    let Damage {
        kind,
        base_amount,
        source,
    } = trigger.event().1.clone();

    // +--------------------------------------------------------------+
    // |                  Start calculating damage                    |
    // +--------------------------------------------------------------+

    let raw_damage = match source {
        DamageSource::Realtime(id) => {
            let default_stats = CurrentStats::default();
            let stats: &CurrentStats = index
                .get(&id)
                .and_then(|entity| query.get(*entity).ok())
                .unwrap_or(&default_stats);

            calculate_raw_damage(&kind, base_amount, stats)
        }
        DamageSource::Snapshot(stats) => calculate_raw_damage(&kind, base_amount, &stats),
    };

    let damage_reduction = calculate_damage_reduction(&kind, base_amount, attackee_stats);

    // ======= Apply damage =======
    // Reborrow a mutable reference to the attackee's stats
    let mut attackee_stats = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
        .unwrap();

    attackee_stats.health -= raw_damage * damage_reduction;
}

pub fn apply_effect(
    trigger: Trigger<ApplyEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    let godot_instance_id = trigger.event().0;
    let effect = trigger.event().1.clone();
    let Some((mut effects, mut timer)) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    else {
        return;
    };

    let effect_id = Uuid::new_v4();

    match effect.duration {
        EffectDuration::Permanent => {
            // nop
        }
        EffectDuration::Temporary(dur) => {
            timer.insert(effect_id, dur);
        }
        EffectDuration::Instant => {
            timer.insert(effect_id, 0.0);
        }
    }

    effects.insert(effect_id, effect);
}

pub fn remove_effect(
    trigger: Trigger<RemoveEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    let godot_instance_id = trigger.event().0;
    let effect_id = trigger.event().1;

    let Some((mut effects, mut timer)) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    else {
        return;
    };

    timer.remove(&effect_id);
    effects.remove(&effect_id);
}

pub fn remove_effects(
    trigger: Trigger<RemoveEffectsEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    let godot_instance_id = trigger.event().0;
    let efts: Vec<Uuid> = trigger.event().1.clone();

    let Some((mut effects, mut timer)) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    else {
        return;
    };

    efts.iter().for_each(|effect_id| {
        timer.remove(effect_id);
        effects.remove(effect_id);
    });
}

pub fn register_entity(
    trigger: Trigger<RegisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {
    let godot_instance_id = trigger.event().0;
    let entity = cmd.spawn(EntityBundle::default()).id();
    index.insert(godot_instance_id, entity);
}

pub fn unregister_entity(
    trigger: Trigger<RegisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {
    let godot_instance_id = trigger.event().0;
    if let Some(entity) = index.remove(&godot_instance_id) {
        cmd.entity(entity).despawn()
    }
}
