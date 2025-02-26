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
use bevy_ecs::event::{event_update_system, EventRegistry};
use event::{
    ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, TakeDamageEvent,
    UnregisterEntityEvent,
};

use crate::node::PackedEntity;
use bevy_ecs::prelude::{Entity, Schedule, World};

use godot::prelude::*;

use godot::classes::Engine;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

struct BattleSystemExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BattleSystemExtension {}

macro_rules! gd_result_try {
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()).into(),
        }
    }};
}

define_mapping! {
    #[derive(Debug)]
    GodotResult => ((String, bool));
}

impl GodotResult {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self((msg.into(), false))
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self((msg.into(), true))
    }
}

impl From<Result<String, String>> for GodotResult {
    fn from(value: Result<String, String>) -> Self {
        match value {
            Ok(v) => Self((v, false)),
            Err(v) => Self((v, true)),
        }
    }
}

impl GodotConvert for GodotResult {
    type Via = Dictionary;
}

impl FromGodot for GodotResult {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let message = via
            .get("message")
            .map(|value| value.try_to::<String>())
            .ok_or(ConvertError::new("Missing \"message\" field"))??;
        let is_error = via
            .get("is_error")
            .map(|value| value.try_to::<bool>())
            .transpose()?
            .unwrap_or_default();

        Ok(Self((message, is_error)))
    }
}

impl ToGodot for GodotResult {
    type ToVia<'v> = Dictionary;

    fn to_godot(&self) -> Self::ToVia<'_> {
        let (message, is_error): &(String, bool) = self.deref();

        dict! {
            "message": message.to_string(),
            "is_error": *is_error,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleSystem {
    pub world: World,
    pub schedule: Schedule,

    base: Base<Node>,
}

#[godot_api]
impl INode for BattleSystem {
    fn init(base: Base<Self::Base>) -> Self {
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
        Self {
            world,
            schedule,
            base,
        }
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
    fn cmd_print_entities(&self) -> GodotResult {
        let map = self.world.resource::<GodotInstanceIdMap>();

        map.0
            .keys()
            .map(|k| k.to_string())
            .reduce(|a, b| a + "\n" + &b)
            .ok_or(String::from("<EMPTY>"))
            .into()
    }

    #[func]
    fn cmd_get_components(&mut self, instance_id: String) -> GodotResult {
        let entity = match self.get_entity(instance_id) {
            Ok(v) => v,
            Err(e) => return Err(e).into(),
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
            return Err(String::from("Entity not found")).into();
        };

        GodotResult::ok(format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            stats, weapon, eq1, eq2, eq3, eq4, effects
        ))
    }

    #[func]
    fn cmd_kill_entity(&self, instance_id: String) -> GodotResult {
        let mut gd_entity: Gd<node::Entity> =
            Gd::from_instance_id(InstanceId::from_i64(gd_result_try!(instance_id.parse())));

        gd_entity.bind_mut().on_entity_died();

        GodotResult::ok(format!("Killed entity: {}", gd_entity))
    }

    #[func(gd_self)]
    fn cmd_spawn_entity(this: Gd<Self>, scene_name: String) -> GodotResult {
        let scene: Gd<PackedScene> =
            gd_result_try!(gd_result_try!(PackedEntity::from_str(&scene_name)).try_into());

        if let Some(entity) = scene.instantiate() {
            let mut battle_scene = this.get_node_as::<Node>("/root/BattleScene");

            battle_scene.add_child(&entity);

            GodotResult::ok(format!("Spawned entity: {}", entity.instance_id()))
        } else {
            GodotResult::err(format!("Cannot spawn this type of entity: {}", scene_name))
        }
    }
}

/// Helper
impl BattleSystem {
    fn get_entity(&self, instance_id: String) -> Result<Entity, String> {
        let instance_id = match instance_id.parse() {
            Ok(entity_id) => InstanceId::from_i64(entity_id),
            Err(e) => return Err(e.to_string()),
        };

        self.world
            .resource::<GodotInstanceIdMap>()
            .get(&instance_id)
            .copied()
            .ok_or(format!("Failed to find entity {}", instance_id))
    }
}

/// Public APIs
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
