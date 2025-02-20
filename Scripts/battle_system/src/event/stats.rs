use crate::component::{Damage, ModifierEffects};
use bevy_ecs::event::Event;

use godot::prelude::InstanceId as GodotInstanceId;
#[derive(Event)]
pub struct TakeDamageEvent(pub GodotInstanceId, pub Damage);

use bevy_ecs::{
    observer::Trigger,
    system::{Query, Res, ResMut},
};
use enumset::EnumSet;
use godot::global::godot_print;

use crate::resource::EntitySnapshot;
use crate::{
    component::{
        CurrentStats, DamageSource, DamageTag, Equipment1, Equipment2, Equipment3, Equipment4,
        Weapon,
    },
    resource::{EntitySnapshotMap, GodotInstanceIdMap},
};

// ========================================================
// TODO: Implement the following functions

#[allow(clippy::too_many_arguments, unused_variables)]
fn calculate_damage(
    kind: &EnumSet<DamageTag>,
    base_amount: f64,

    attacker: Option<EntitySnapshot>,
    attackee: EntitySnapshot,
) -> f64 {
    #[cfg(debug_assertions)]
    godot_print!("calculate_raw_damage");

    todo!();
}

// ========================================================

pub fn make_snapshot(
    components: (
        &CurrentStats,
        &Weapon,
        &Equipment1,
        &Equipment2,
        &Equipment3,
        &Equipment4,
        &ModifierEffects,
    ),
) -> EntitySnapshot {
    let effects = components.6.iter().fold(Vec::new(), |mut v, (_, efts)| {
        v.extend(efts.clone());
        v
    });
    EntitySnapshot {
        stats: components.0.clone(),
        weapon: components.1.clone(),
        eq1: components.2.clone(),
        eq2: components.3.clone(),
        eq3: components.4.clone(),
        eq4: components.5.clone(),
        effects,
    }
}

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
        &ModifierEffects,
    )>,
) {
    #[cfg(debug_assertions)]
    godot_print!("take_damage");

    let (attackee_instance_id, damage) = (trigger.event().0, trigger.event().1.clone());

    let Some(entity) = instance_map.get(&attackee_instance_id) else {
        return;
    };

    let Ok(attackee_components) = query.get(*entity) else {
        return;
    };

    let attackee_components = make_snapshot(attackee_components);

    let attacker_components: Option<EntitySnapshot> = match damage.source {
        DamageSource::Realtime(id) => instance_map
            .get(&id)
            .and_then(|entity| query.get(*entity).ok())
            .map(make_snapshot),
        DamageSource::Snapshot(snapshot_id) => {
            snapshot_map
                .get_mut(&snapshot_id)
                .map(|(snapshot, ref_count)| {
                    *ref_count -= 1;
                    snapshot.clone()
                })
        }
    };

    // Calculate damage
    let damage = calculate_damage(
        &damage.kind,
        damage.base_amount,
        attacker_components,
        attackee_components,
    );

    if let Ok(mut attackee) = query.get_mut(*entity) {
        attackee.0.health -= damage;
    }
}
