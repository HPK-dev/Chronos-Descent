use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashSet;

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

#[derive(Debug, strum::EnumString, strum::Display)]
pub enum EquipmentStats {}

#[derive(Debug, Default)]
pub struct Equipment {
    pub stats: FxHashSet<EquipmentStats>,
    pub perks: EnumSet<EquipmentTag>,
}
