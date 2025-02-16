use bevy_ecs::{
    observer::Trigger,
    system::{Commands, Query, Res, ResMut},
};
use enumset::EnumSet;
use godot::global::godot_print;
use uuid::Uuid;

use crate::{
    bundle::EntityBundle,
    component::{
        CurrentStats, DamageSource, DamageTag, EffectDuration, Effects, EffectsTimer, Equipment,
        Equipment1, Equipment2, Equipment3, Equipment4, Weapon,
    },
    event::{
        ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, RemoveEffectsEvent,
        TakeDamageEvent,
    },
    resource::{EntitySnapshotMap, GodotInstanceIdMap},
};

struct CalculationRequiredComponent<'a> {
    stats: &'a CurrentStats,
    weapon: &'a Weapon,
    eq1: &'a Equipment1,
    eq2: &'a Equipment2,
    eq3: &'a Equipment3,
    eq4: &'a Equipment4,
}

impl<'a>
    From<(
        &'a CurrentStats,
        &'a Weapon,
        &'a Equipment1,
        &'a Equipment2,
        &'a Equipment3,
        &'a Equipment4,
    )> for CalculationRequiredComponent<'a>
{
    fn from(
        (stats, weapon, eq1, eq2, eq3, eq4): (
            &'a CurrentStats,
            &'a Weapon,
            &'a Equipment1,
            &'a Equipment2,
            &'a Equipment3,
            &'a Equipment4,
        ),
    ) -> Self {
        Self {
            stats,
            weapon,
            eq1,
            eq2,
            eq3,
            eq4,
        }
    }
}

// ========================================================
// TODO: Implement the following functions

#[allow(clippy::too_many_arguments, unused_variables)]
fn calculate_raw_damage(
    kind: &EnumSet<DamageTag>,
    base_amount: f64,

    CalculationRequiredComponent {
        stats,
        weapon,
        eq1,
        eq2,
        eq3,
        eq4,
    }: CalculationRequiredComponent,
) -> f64 {
    #[cfg(debug_assertions)]
    godot_print!("calculate_raw_damage");

    todo!();
}

#[allow(clippy::too_many_arguments, unused_variables)]
fn calculate_damage_reduction(
    kind: &EnumSet<DamageTag>,
    base_amount: f64,
    CalculationRequiredComponent {
        stats,
        weapon,
        eq1,
        eq2,
        eq3,
        eq4,
    }: CalculationRequiredComponent,
) -> f64 {
    #[cfg(debug_assertions)]
    godot_print!("calculate_damage_reduction");

    todo!();
}

// ========================================================

pub fn take_damage(
    trigger: Trigger<TakeDamageEvent>,
    instance_map: Res<GodotInstanceIdMap>,
    mut snapshot_map: ResMut<EntitySnapshotMap>,
    mut query: Query<(
        &mut CurrentStats,
        &Weapon,
        &Equipment1,
        &Equipment2,
        &Equipment3,
        &Equipment4,
    )>,
) {
    #[cfg(debug_assertions)]
    godot_print!("take_damage");

    let (godot_instance_id, damage) = (trigger.event().0, trigger.event().1.clone());

    let Some(entity) = instance_map.get(&godot_instance_id) else {
        return;
    };

    let Ok(attackee_components) = query.get(*entity) else {
        return;
    };

    let attackee_equipment = CalculationRequiredComponent::from((
        attackee_components.0,
        attackee_components.1,
        attackee_components.2,
        attackee_components.3,
        attackee_components.4,
        attackee_components.5,
    ));

    let attacker_components: Option<(
        &CurrentStats,
        &Weapon,
        &Equipment1,
        &Equipment2,
        &Equipment3,
        &Equipment4,
    )> = match damage.source {
        DamageSource::Realtime(id) => instance_map
            .get(&id)
            .and_then(|entity| query.get(*entity).ok()),
        DamageSource::Snapshot(snapshot_id) => {
            snapshot_map
                .get_mut(&snapshot_id)
                .and_then(|(entity, ref_count)| {
                    *ref_count -= 1;
                    query.get(*entity).ok()
                })
        }
    };

    let default_components: (
        &CurrentStats,
        &Weapon,
        &Equipment1,
        &Equipment2,
        &Equipment3,
        &Equipment4,
    ) = (
        &CurrentStats::default(),
        &Weapon::default(),
        &Equipment::default().into(),
        &Equipment::default().into(),
        &Equipment::default().into(),
        &Equipment::default().into(),
    );

    let attacker_components = attacker_components.unwrap_or(default_components);
    let attacker_equipment = CalculationRequiredComponent::from((
        attacker_components.0,
        attacker_components.1,
        attacker_components.2,
        attacker_components.3,
        attacker_components.4,
        attacker_components.5,
    ));

    // Calculate damage
    let raw_damage = calculate_raw_damage(&damage.kind, damage.base_amount, attacker_equipment);

    let damage_reduction =
        calculate_damage_reduction(&damage.kind, damage.base_amount, attackee_equipment);

    if let Ok(mut attackee) = query.get_mut(*entity) {
        attackee.0.health -= raw_damage * damage_reduction;
    }
}

pub fn apply_effect(
    trigger: Trigger<ApplyEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    #[cfg(debug_assertions)]
    godot_print!("apply_effect");

    let (godot_instance_id, effect) = (trigger.event().0, trigger.event().1.clone());

    let Some(entity) = index.get(&godot_instance_id) else {
        return;
    };

    let Ok((mut effects, mut timer)) = query.get_mut(*entity) else {
        return;
    };

    let effect_id = Uuid::new_v4();

    match effect.duration {
        EffectDuration::Permanent => (),
        EffectDuration::Temporary(dur) => {
            timer.insert(effect_id, dur);
        }
        EffectDuration::Instant => {
            timer.insert(effect_id, 0.0);
        }
    }

    effects.insert(effect_id, effect);
}

pub fn remove_effects_by_ids(
    ids: impl IntoIterator<Item = Uuid>,
    effects: &mut Effects,
    timer: &mut EffectsTimer,
) {
    for id in ids {
        timer.remove(&id);
        effects.remove(&id);
    }
}

pub fn remove_effect(
    trigger: Trigger<RemoveEffectEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    #[cfg(debug_assertions)]
    godot_print!("remove_effect");

    let (godot_instance_id, effect_id) = (trigger.event().0, trigger.event().1);

    if let Some((mut effects, mut timer)) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    {
        remove_effects_by_ids(std::iter::once(effect_id), &mut effects, &mut timer);
    }
}

pub fn remove_effects(
    trigger: Trigger<RemoveEffectsEvent>,
    index: Res<GodotInstanceIdMap>,
    mut query: Query<(&mut Effects, &mut EffectsTimer)>,
) {
    #[cfg(debug_assertions)]
    godot_print!("remove_effects");

    let (godot_instance_id, effect_ids) = (trigger.event().0, trigger.event().1.clone());

    if let Some((mut effects, mut timer)) = index
        .get(&godot_instance_id)
        .and_then(|entity| query.get_mut(*entity).ok())
    {
        remove_effects_by_ids(effect_ids, &mut effects, &mut timer);
    }
}

pub fn register_entity(
    trigger: Trigger<RegisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {
    #[cfg(debug_assertions)]
    godot_print!("register_entity");

    let godot_instance_id = trigger.event().0;
    let entity = cmd.spawn(EntityBundle::from(godot_instance_id)).id();
    index.insert(godot_instance_id, entity);
}

pub fn unregister_entity(
    trigger: Trigger<RegisterEntityEvent>,
    mut cmd: Commands,
    mut index: ResMut<GodotInstanceIdMap>,
) {
    #[cfg(debug_assertions)]
    godot_print!("unregister_entity");

    let godot_instance_id = trigger.event().0;
    if let Some(entity) = index.remove(&godot_instance_id) {
        cmd.entity(entity).despawn();
    }
}
