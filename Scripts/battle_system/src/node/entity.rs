use crate::get_battle_system_singleton;
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
         get_battle_system_singleton()
            .bind_mut()
            .register_entity_with_instance_id(self.base().instance_id());
    }
}
