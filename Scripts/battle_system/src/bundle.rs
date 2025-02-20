use bevy_ecs::prelude::*;
use godot::obj::InstanceId;

use crate::component::{
    CrowdControlEffects, CurrentStats, EffectsQueue, Equipment1, Equipment2, Equipment3,
    Equipment4, GodotInstanceId, ModifierEffects, TickEffects, Weapon,
};

#[derive(Bundle, Default)]
pub struct EntityBundle {
    pub instance_id: GodotInstanceId,
    pub current_stats: CurrentStats,

    pub weapon: Weapon,
    pub equipment1: Equipment1,
    pub equipment2: Equipment2,
    pub equipment3: Equipment3,
    pub equipment4: Equipment4,

    pub effects_queue: EffectsQueue,
    pub tick_effects: TickEffects,
    pub modifier_effects: ModifierEffects,
    pub crowd_control_effects: CrowdControlEffects,
}

impl From<InstanceId> for EntityBundle {
    fn from(instance_id: InstanceId) -> Self {
        Self {
            instance_id: GodotInstanceId(instance_id),
            ..Default::default()
        }
    }
}
