use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashSet;

use crate::define_mapping;

#[derive(strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

#[derive(strum::EnumString, strum::Display, Clone)]
pub enum EquipmentStats {}

#[derive(Default, Clone)]
pub struct Equipment {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
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
