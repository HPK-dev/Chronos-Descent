use enumset::{EnumSet, EnumSetType};

#[derive(Debug, strum::EnumString, strum::Display, EnumSetType)]
pub enum EquipmentTag {}

pub struct EquipmentStats {}

pub struct Equipment {
    pub stats: EquipmentStats,
    pub perks: EnumSet<EquipmentTag>,
}
