use crate::define_mapping;
use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter};

#[derive(strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

#[derive(strum::EnumString, strum::Display, Clone)]
pub enum EquipmentStats {}

#[derive(Clone)]
pub struct Equipment {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
    pub id: String,
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            id: String::from("Unknown"),
            stats: FxHashSet::default(),
            perks: EnumSet::new(),
        }
    }
}

impl Display for Equipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Equipment: {}
        ======================
        Stats: 
        {}
        Perks:
        {}
        "#,
            self.id,
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

define_mapping! {
    #[derive(Clone, Component, Default)]
    Equipment1 => (Equipment);
    #[derive(Clone, Component, Default)]
    Equipment2 => (Equipment);
    #[derive(Clone, Component, Default)]
    Equipment3 => (Equipment);
    #[derive(Clone, Component, Default)]
    Equipment4 => (Equipment);
}

impl Display for Equipment1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl Display for Equipment2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl Display for Equipment3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl Display for Equipment4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
