pub mod bundle;
pub mod component;
pub mod event;
pub mod node;
pub mod resource;
pub mod system;
pub mod utils;

use crate::component::{
    CurrentStats, EffectsTimer, Equipment1, Equipment2, Equipment3, Equipment4, ModifierEffects,
    Weapon,
};
use crate::resource::{EntitySnapshotMap, GodotInstanceIdMap, GodotTimeDelta, GodotTimeScale};
use bevy_ecs::{
    event::{event_update_system, EventRegistry},
    prelude::*,
};
use event::{
    ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, TakeDamageEvent,
    UnregisterEntityEvent,
};

use crate::event::make_snapshot;
use godot::{
    classes::Engine,
    global::{godot_print, godot_print_rich},
    obj::{Base, InstanceId},
    prelude::{gdextension, godot_api, ExtensionLibrary, GodotClass, INode},
};
use uuid::Uuid;

struct BattleSystemExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BattleSystemExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleSystem {
    pub world: World,
    pub schedule: Schedule,
}

#[godot_api]
impl INode for BattleSystem {
    fn init(_: Base<Self::Base>) -> Self {
        godot_print_rich!(r#"[font_size=30] Battle system is initilized! [/font_size]"#);
        let schedule = Schedule::default();
        let world = World::new();

        Self { world, schedule }
    }

    fn physics_process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.world.resource_mut::<GodotTimeDelta>().0 = delta as f32;
        self.schedule.run(&mut self.world);
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        let world = &mut self.world;
        let schedule = &mut self.schedule;

        world.init_resource::<GodotTimeDelta>();
        world.init_resource::<GodotTimeScale>();
        world.init_resource::<GodotInstanceIdMap>();
        world.init_resource::<EntitySnapshotMap>();
        world.init_resource::<EffectsTimer>();

        EventRegistry::register_event::<RegisterEntityEvent>(world);
        EventRegistry::register_event::<UnregisterEntityEvent>(world);
        EventRegistry::register_event::<ApplyEffectEvent>(world);
        EventRegistry::register_event::<RemoveEffectEvent>(world);
        EventRegistry::register_event::<TakeDamageEvent>(world);

        world.add_observer(event::apply_effect);
        world.add_observer(event::remove_effect);

        schedule
            .add_systems(event_update_system)
            .add_systems(system::effect_timer_update)
            .add_systems(system::tick_effect_update)
            .add_systems(system::snapshot_ref_count_update);
    }
}

#[godot_api]
impl BattleSystem {
    #[func]
    fn set_time_scale(&mut self, time_scale: f64) {
        self.world.resource_mut::<GodotTimeScale>().0 = time_scale as f32;
    }
}

impl BattleSystem {
    pub fn register_entity(&mut self, instance_id: InstanceId) {
        self.world.trigger(RegisterEntityEvent(instance_id));
        godot_print!("Register entity: {:?}", instance_id);
    }

    pub fn unregister_entity(&mut self, instance_id: InstanceId) {
        self.world.trigger(UnregisterEntityEvent(instance_id));
        godot_print!("Unregister entity: {:?}", instance_id);
    }

    pub fn new_snapshot(&mut self, instance_id: &InstanceId, ref_count: usize) -> Option<Uuid> {
        let origin_entity = {
            let instance_map = self.world.resource::<GodotInstanceIdMap>();
            instance_map.get(instance_id).copied()?
        };

        let components = {
            let mut query = self.world.query::<(
                &CurrentStats,
                &Weapon,
                &Equipment1,
                &Equipment2,
                &Equipment3,
                &Equipment4,
                &ModifierEffects,
            )>();
            query.get(&self.world, origin_entity).unwrap()
        };

        let copied_entity = make_snapshot(components);

        let id = Uuid::new_v4();
        let mut snapshot_map = self.world.resource_mut::<EntitySnapshotMap>();
        snapshot_map.insert(id, (copied_entity, ref_count));

        Some(id)
    }
}
