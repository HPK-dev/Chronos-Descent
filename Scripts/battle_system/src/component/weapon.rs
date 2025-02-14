use super::{EquipmentStats, EquipmentTag};
use bevy_ecs::prelude::*;
use enumset::EnumSet;
use rustc_hash::FxHashSet;

pub enum Skill {
    None,
}

#[derive(Component)]
pub struct Weapon {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
    pub skill: Skill,
}

impl Weapon {
    pub fn empty() -> Self {
        Self {
            stats: FxHashSet::default(),
            perks: EnumSet::new(),
            skill: Skill::None,
        }
    }
}
