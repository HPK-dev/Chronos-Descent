use bevy_ecs::prelude::*;
use godot::obj::InstanceId;

use crate::component::{
    BaseStats, CurrentStats, Equipment1, Equipment2, Equipment3, Equipment4, GodotInstanceId,
    Weapon,
};

#[derive(Bundle)]
pub struct EntityBundle {
    pub instance_id: GodotInstanceId,
    pub current_stats: CurrentStats,
    pub base_stats: BaseStats,
    pub weapon: Weapon,
    pub equipment1: Equipment1,
    pub equipment2: Equipment2,
    pub equipment3: Equipment3,
    pub equipment4: Equipment4,
}

impl Default for EntityBundle {
    fn default() -> Self {
        Self {
            instance_id: GodotInstanceId(InstanceId::from_i64(i64::MIN)),
            current_stats: CurrentStats::default(),
            base_stats: BaseStats::default(),
            weapon: Weapon::default(),
            equipment1: Equipment1::default(),
            equipment2: Equipment2::default(),
            equipment3: Equipment3::default(),
            equipment4: Equipment4::default(),
        }
    }
}

impl From<InstanceId> for EntityBundle {
    fn from(instance_id: InstanceId) -> Self {
        Self {
            instance_id: GodotInstanceId(instance_id),
            ..Default::default()
        }
    }
}
