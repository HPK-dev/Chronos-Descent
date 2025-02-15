use bevy_ecs::prelude::*;

use crate::component::{
    Armor, Artifact1, Artifact2, Artifact3, BaseStats, CurrentStats, InstanceId, Weapon,
};

#[derive(Bundle)]
pub struct EntityBundle {
    pub instance_id: InstanceId,
    pub current_stats: CurrentStats,
    pub base_stats: BaseStats,
    pub weapon: Weapon,
    pub armor: Armor,
    pub artifact1: Artifact1,
    pub artifact2: Artifact2,
    pub artifact3: Artifact3,
}
