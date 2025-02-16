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
    pub armor: Equipment1,
    pub artifact1: Equipment4,
    pub artifact2: Equipment2,
    pub artifact3: Equipment3,
}

impl From<InstanceId> for EntityBundle {
    fn from(instance_id: InstanceId) -> Self {
        Self {
            instance_id: GodotInstanceId(instance_id),
            current_stats: CurrentStats::default(),
            base_stats: BaseStats::default(),
            weapon: Weapon::default(),
            armor: Equipment1::default(),
            artifact1: Equipment4::default(),
            artifact2: Equipment2::default(),
            artifact3: Equipment3::default(),
        }
    }
}
