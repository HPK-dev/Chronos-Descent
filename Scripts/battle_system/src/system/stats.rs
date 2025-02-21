use crate::component::{CurrentStats, GodotInstanceId};
use crate::node;
use crate::resource::EntitySnapshotMap;
use bevy_ecs::prelude::*;
use godot::global::godot_print;
use godot::prelude::Gd;

pub fn current_stats_update(
    query: Query<(Entity, &CurrentStats, &GodotInstanceId), Changed<CurrentStats>>,
    mut cmd: Commands,
) {
    #[cfg(debug_assertions)]
    godot_print!("Current stats update!");

    for (entity, stats, id) in query.iter() {
        if stats.health <= 0.0 {
            cmd.entity(entity).despawn();
            let gd_obj: Gd<node::Entity> = Gd::from_instance_id(**id);
            gd_obj.callable("on_entity_died").call(&[]);
        }
    }

    // TODO: maybe handle other situations?
}

pub fn snapshot_ref_decrease(mut snapshot_map: ResMut<EntitySnapshotMap>) {
    if snapshot_map.is_changed() {
        #[cfg(debug_assertions)]
        godot_print!("Snapshot ref count update!");

        let mut outdated_snapshot = Vec::new();

        snapshot_map.retain(|id, (_, count)| {
            if *count == 0 {
                outdated_snapshot.push(*id);
                false
            } else {
                true
            }
        });

        for id in outdated_snapshot {
            snapshot_map.remove(&id);
        }
    }
}
