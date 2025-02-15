use bevy_ecs::{
    observer::Trigger,
    system::{Commands, Query, Res, ResMut},
};
use uuid::Uuid;

use crate::{
    bundle::EntityBundle,
    component::{
        CurrentStats, Damage, DamageSource, Effect, EffectDuration, Effects, EffectsTimer,
    },
    event::{
        ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, RemoveEffectsEvent,
        TakeDamageEvent,
    },
    resource::GodotInstanceIdMap,
};

pub fn take_damage(
    trigger: Trigger<TakeDamageEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<&mut CurrentStats>,
) {
    let godot_instance_id = trigger.event().0;

    let Some(mut attackee_stats) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    else {
        return;
    };

    let Damage {
        kind,
        base_amount,
        source,
    } = trigger.event().1.clone();

    let attacker_stats = match source {
        DamageSource::Realtime(id) => index
            .get(&id)
            .and_then(|entity| query.get(*entity).ok())
            .cloned(),
        DamageSource::Snapshot(stats) => Some(stats),
    };

    // +--------------------------------------------------------------+
    // |                  Start calculating damage                    |
    // +--------------------------------------------------------------+

    let damage = if let Some(attacker_stats) = attacker_stats {
        // ============ Calculate base damage ============
        let base_damage: f64 = todo!();

        // ============ Calculate crit damage ============
        let crit_damage: f64 = todo!();

        // ============ Apply modifiers ============
        (base_damage * crit_damage * attacker_stats.multiplicative_multiplier
            + attacker_stats.additive_multiplier)
            * attacker_stats.bouns_multiplier
    } else {
        base_amount
    };

    // ============ Calculate damage reduction ============
    let damage_reduction: f64 = todo!();

    // ============ Calculate final damage ============
    let final_damage = damage * damage_reduction;

    // ============ Apply damage ============
    attackee_stats.health -= final_damage;
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
        timer.remove(&effect_id);
        effects.remove(&effect_id);
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
