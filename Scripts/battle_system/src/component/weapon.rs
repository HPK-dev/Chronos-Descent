use super::{EquipmentStats, EquipmentTag};
use bevy_ecs::prelude::*;
use enumset::EnumSet;
use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter};
use strum::Display;

#[derive(Clone, Display)]
pub enum Skill {
    None,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
    pub skill: Skill,
    pub id: String,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            stats: FxHashSet::default(),
            perks: EnumSet::new(),
            skill: Skill::None,
            id: String::from("Unknown"),
        }
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Weapon: {}
        ======================
        Skill: {}
        Stats: 
        {}
        Perks:
        {}
        "#,
            self.id,
            self.skill,
            self.stats
                .iter()
                .map(|e| e.to_string())
                .reduce(|a, b| a + "\n" + &b)
                .unwrap_or("<EMPTY>".into()),
            self.perks
                .iter()
                .map(|e| e.to_string())
                .reduce(|a, b| a + "\n" + &b)
                .unwrap_or("<EMPTY>".into()),
        )
    }
}
