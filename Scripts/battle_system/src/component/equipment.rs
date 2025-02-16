use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashSet;
use std::ops::{Deref, DerefMut};

#[derive(strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

#[derive(strum::EnumString, strum::Display, Clone)]
pub enum EquipmentStats {}

#[derive(Default, Clone)]
pub struct Equipment {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
}

macro_rules! new_type {
    ($name:ident, $origin:ident) => {
        #[derive(Component, Clone)]
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

        impl Default for $name {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl From<$origin> for $name {
            fn from(origin: $origin) -> Self {
                Self(origin)
            }
        }
    };
}

new_type!(Equipment1, Equipment);
new_type!(Equipment2, Equipment);
new_type!(Equipment3, Equipment);
new_type!(Equipment4, Equipment);
