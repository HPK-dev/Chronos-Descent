use crate::component::CurrentStats;
use crate::resource::EntitySnapshotMap;
use bevy_ecs::prelude::*;
use godot::global::godot_print;

pub fn current_stats_update(query: Query<Entity, Changed<CurrentStats>>) {
    #[cfg(debug_assertions)]
    godot_print!("Current stats update!");

    todo!();
}

pub fn snapshot_ref_count_update(mut snapshot_map: ResMut<EntitySnapshotMap>) {
    if snapshot_map.is_changed() {
        #[cfg(debug_assertions)]
        godot_print!("Snapshot ref count update!");

        let mut outdated_snapshot = None;

        snapshot_map.retain(|id, (_, count)| {
            if *count == 0 {
                outdated_snapshot = Some(*id);
                false
            } else {
                true
            }
        });

        if let Some(id) = outdated_snapshot {
            snapshot_map.remove(&id);
        }
    }
}
