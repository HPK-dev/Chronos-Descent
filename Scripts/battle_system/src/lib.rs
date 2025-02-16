pub mod bundle;
pub mod component;
pub mod event;
pub mod node;
pub mod resource;
pub mod system;

use bevy_ecs::{
    event::{event_update_system, EventRegistry},
    prelude::*,
};
use event::{
    ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, RemoveEffectsEvent, TakeDamageEvent,
    UnregisterEntityEvent,
};
use godot::prelude::{gdextension, godot_api, Base, ExtensionLibrary, Gd, GodotClass, INode, Node};

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

        let world = &mut self.world;
        let schedule = &mut self.schedule;

        // Setup systems
        world.init_resource::<resource::GodotTimeDelta>();
        world.init_resource::<resource::GodotTimeScale>();
        world.init_resource::<resource::GodotInstanceIdMap>();
        world.init_resource::<resource::EntitySnapshotMap>();

        EventRegistry::register_event::<RegisterEntityEvent>(world);
        EventRegistry::register_event::<UnregisterEntityEvent>(world);
        EventRegistry::register_event::<ApplyEffectEvent>(world);
        EventRegistry::register_event::<RemoveEffectEvent>(world);
        EventRegistry::register_event::<RemoveEffectsEvent>(world);
        EventRegistry::register_event::<TakeDamageEvent>(world);

        world.add_observer(system::apply_effect);
        world.add_observer(system::remove_effect);
        world.add_observer(system::remove_effects);

        schedule.add_systems(event_update_system);
        schedule.add_systems(system::effect_timer_update);
        schedule.add_systems(system::effects_changed_update);
    }

    fn physics_process(&mut self, delta: f64) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        self.world.resource_mut::<resource::GodotTimeDelta>().0 = delta;
        self.schedule.run(&mut self.world);
    }
}

#[godot_api]
impl BattleSystem {
    #[func]
    fn register_entity(&mut self, entity: Gd<node::Entity>) {
        let instance_id = entity.instance_id();
        self.world.trigger(RegisterEntityEvent(instance_id));
    }

    #[func]
    fn unregister_entity(&mut self, entity: Gd<node::Entity>) {
        let instance_id = entity.instance_id();
        self.world
            .trigger(event::UnregisterEntityEvent(instance_id));
    }

    #[func]
    fn set_time_scale(&mut self, time_scale: f64) {
        self.world.resource_mut::<resource::GodotTimeScale>().0 = time_scale;
    }
}
