pub mod component;
pub mod entity;
pub mod system;

use bevy_ecs::prelude::*;
use godot::prelude::*;

macro_rules! apply {
    ($obj:ident. {
        $(. $func:ident ( $(args:ident),* ) ),* $(,)*
    }) => {{
        $(
            $obj.$func( $( $args ),* );
        )*
    }};
}

struct BattleSystemExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BattleSystemExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleSystem {
    world: World,
    schedule: Schedule,
}

#[godot_api]
impl INode for BattleSystem {
    fn init(_: Base<Node>) -> Self {
        let schedule = Schedule::default();
        let world = World::new();
        Self { world, schedule }
    }

    fn ready(&mut self) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        // Setup systems
        apply!(world .{

        })
    }

    fn physics_process(&mut self, _delta: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        self.schedule.run(&mut self.world);
    }
}
