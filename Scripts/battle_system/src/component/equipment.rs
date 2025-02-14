use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashSet;
use std::ops::{Deref, DerefMut};

#[derive(strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

#[derive(strum::EnumString, strum::Display)]
pub enum EquipmentStats {}

#[derive(Default)]
pub struct Equipment {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
}

macro_rules! new_type {
    ($name:ident, $origin:ident) => {
        #[derive(Component)]
        pub struct $name(pub $origin);

        impl Deref for $name {
            type Target = $origin;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

new_type!(Armor, Equipment);
new_type!(Artifact1, Equipment);
new_type!(Artifact2, Equipment);
new_type!(Artifact3, Equipment);
