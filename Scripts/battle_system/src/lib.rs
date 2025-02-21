pub mod bundle;
pub mod component;
pub mod event;
pub mod node;
pub mod resource;
pub mod system;
pub mod utils;

use crate::component::{
    CurrentStats, EffectsQueue, EffectsTimer, Equipment1, Equipment2, Equipment3, Equipment4,
    ModifierEffects, Weapon,
};
use crate::event::make_snapshot;
use crate::resource::{EntitySnapshotMap, GodotInstanceIdMap, GodotTimeDelta, GodotTimeScale};
use bevy_ecs::{
    event::{event_update_system, EventRegistry},
    prelude::*,
};
use event::{
    ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, TakeDamageEvent,
    UnregisterEntityEvent,
};
use godot::{
    classes::Engine,
    global::godot_print_rich,
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
        let mut schedule = Schedule::default();
        let mut world = World::new();

        world.init_resource::<GodotTimeDelta>();
        world.init_resource::<GodotTimeScale>();
        world.init_resource::<GodotInstanceIdMap>();
        world.init_resource::<EntitySnapshotMap>();
        world.init_resource::<EffectsTimer>();

        EventRegistry::register_event::<RegisterEntityEvent>(&mut world);
        EventRegistry::register_event::<UnregisterEntityEvent>(&mut world);
        EventRegistry::register_event::<ApplyEffectEvent>(&mut world);
        EventRegistry::register_event::<RemoveEffectEvent>(&mut world);
        EventRegistry::register_event::<TakeDamageEvent>(&mut world);

        world.add_observer(event::apply_effect);
        world.add_observer(event::remove_effect);
        world.add_observer(event::register_entity);
        world.add_observer(event::unregister_entity);

        schedule
            .add_systems(event_update_system)
            .add_systems(system::effect_timer_update)
            .add_systems(system::tick_effect_update)
            .add_systems(system::snapshot_ref_decrease);

        godot_print_rich!(r#"[font_size=30] Battle system is initilized! [/font_size]"#);
        Self { world, schedule }
    }

    fn physics_process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.world.resource_mut::<GodotTimeDelta>().0 = delta as f32;
        self.schedule.run(&mut self.world);
    }
}

#[godot_api]
impl BattleSystem {
    /// Set battle system's time speed scale.  
    /// For example, set battle system's timescale to 0.5
    /// will cause player to move as normal but effect duration extend to 2x long.
    #[func]
    fn set_timescale(&mut self, time_scale: f64) {
        self.world.resource_mut::<GodotTimeScale>().0 = time_scale as f32;
    }

    #[func]
    fn cmd_print_entities(&self) -> String {
        let map = self.world.resource::<GodotInstanceIdMap>();

        map.0
            .keys()
            .map(|k| k.to_string())
            .reduce(|a, b| a + "\n" + &b)
            .unwrap_or(String::from("<EMPTY>"))
    }

    #[func]
    fn cmd_get_components(&mut self, instance_id: String) -> String {
        let instance_id = match instance_id.parse() {
            Ok(entity_id) => InstanceId::from_i64(entity_id),
            Err(e) => return format!("{}", e),
        };

        let Some(entity) = self
            .world
            .resource::<GodotInstanceIdMap>()
            .get(&instance_id)
            .copied()
        else {
            return "Entity not found".into();
        };

        let mut query = self.world.query::<(
            &CurrentStats,
            &Weapon,
            &Equipment1,
            &Equipment2,
            &Equipment3,
            &Equipment4,
            &EffectsQueue,
        )>();

        let Ok((stats, weapon, eq1, eq2, eq3, eq4, effects)) = query.get(&self.world, entity)
        else {
            return String::from("Entity not found");
        };

        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            stats, weapon, eq1, eq2, eq3, eq4, effects
        )
    }
}

/// Commands
impl BattleSystem {}

impl BattleSystem {
    pub fn register_entity(&mut self, instance_id: InstanceId) {
        self.world.trigger(RegisterEntityEvent(instance_id));
    }

    pub fn unregister_entity(&mut self, instance_id: InstanceId) {
        self.world.trigger(UnregisterEntityEvent(instance_id));
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
