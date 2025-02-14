use godot::classes::CharacterBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init, base=CharacterBody2D)]
pub struct Entity {
    pub base: Base<CharacterBody2D>,
}
