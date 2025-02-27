use crate::BattleSystem;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;
use strum::EnumString;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Entity {
    pub base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Entity {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let base = self.base_mut();
        let instance_id = base.instance_id().to_i64();
        let mut battle_system: Gd<BattleSystem> =
            base.get_node_as("/root/BattleScene/BattleSystem");

        battle_system.bind_mut().register_entity(instance_id);
    }
}

#[godot_api]
impl Entity {
    #[func(virtual)]
    pub fn on_entity_died(&mut self) {
        godot_print!("{}: I'm died!", self.base().instance_id());
        let mut base = self.base_mut();
        let instance_id = base.instance_id().to_i64();
        let mut battle_system: Gd<BattleSystem> =
            base.get_node_as("/root/BattleScene/BattleSystem");

        battle_system.bind_mut().unregister_entity(instance_id);
        base.queue_free();
    }
}

#[derive(EnumString, strum::Display)]
pub enum PackedEntity {
    Dummy,
}

impl TryFrom<PackedEntity> for Gd<PackedScene> {
    type Error = IoError;

    fn try_from(value: PackedEntity) -> Result<Self, Self::Error> {
        let path = format!("res://Scenes/entity/{}.tscn", value.to_string());
        try_load(&path)
    }
}

/* TODO:
    - shoot projectile
    - entity selector
    - damage indicator
    - pathfinding
*/
