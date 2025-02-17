use crate::BattleSystem;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

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
        let mut base = self.base_mut();
        let mut battle_system = base.get_node_as::<BattleSystem>("%BattleSystem");
        let mut bind = battle_system.bind_mut();
        bind.register_entity_with_instance_id(base.instance_id());
    }
}
