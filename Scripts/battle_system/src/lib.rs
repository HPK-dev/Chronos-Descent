pub mod bundle;
pub mod component;
pub mod event;
pub mod node;
pub mod resource;
pub mod system;

use crate::bundle::EntityBundle;
use crate::component::{CurrentStats, Equipment1, Equipment2, Equipment3, Equipment4, Weapon};
use crate::resource::{EntitySnapshotMap, GodotInstanceIdMap, GodotTimeDelta, GodotTimeScale};
use bevy_ecs::{
    event::{event_update_system, EventRegistry},
    prelude::*,
};
use event::{
    ApplyEffectEvent, RegisterEntityEvent, RemoveEffectEvent, RemoveEffectsEvent, TakeDamageEvent,
    UnregisterEntityEvent,
};

use godot::{
    classes::Engine,
    global::godot_print,
    global::godot_print_rich,
    init::InitLevel,
    obj::InstanceId,
    prelude::{gdextension, godot_api, ExtensionLibrary, Gd, GodotClass, INode},
};
use uuid::Uuid;

pub const BATTLE_SYSTEM_SINGLETON_NAME: &'static str = "BattleSystemSingleton";
pub fn get_battle_system_singleton() -> Gd<BattleSystem> {
    if let Some(singleton) = Engine::singleton().get_singleton(BATTLE_SYSTEM_SINGLETON_NAME) {
        return singleton.cast::<BattleSystem>();
    }

    panic!("WHY BATTLE SYSTEM HAS NOT GET REGISTERED YET!???")
}

struct BattleSystemExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BattleSystemExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            engine.register_singleton(BATTLE_SYSTEM_SINGLETON_NAME, &BattleSystem::new());

            godot_print_rich!(
                r#"[font_size=30] Battle system singleton is registered! [/font_size]"#
            );
            godot_print!("");
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            engine.unregister_singleton(BATTLE_SYSTEM_SINGLETON_NAME);
        }
    }
}

#[derive(GodotClass)]
#[class(no_init,base=Node)]
pub struct BattleSystem {
    world: World,
    schedule: Schedule,
}

impl BattleSystem {
    fn new() -> Gd<Self> {
        let schedule = Schedule::default();
        let world = World::new();
        Gd::from_init_fn(|_| Self { world, schedule })
    }
}

#[godot_api]
impl INode for BattleSystem {
    fn physics_process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.world.resource_mut::<GodotTimeDelta>().0 = delta;
        self.schedule.run(&mut self.world);
    }

    fn ready(&mut self) {
        if godot::classes::Engine::singleton().is_editor_hint() {
            return;
        }

        let world = &mut self.world;
        let schedule = &mut self.schedule;

        // Setup systems
        world.init_resource::<GodotTimeDelta>();
        world.init_resource::<GodotTimeScale>();
        world.init_resource::<GodotInstanceIdMap>();
        world.init_resource::<EntitySnapshotMap>();

        EventRegistry::register_event::<RegisterEntityEvent>(world);
        EventRegistry::register_event::<UnregisterEntityEvent>(world);
        EventRegistry::register_event::<ApplyEffectEvent>(world);
        EventRegistry::register_event::<RemoveEffectEvent>(world);
        EventRegistry::register_event::<RemoveEffectsEvent>(world);
        EventRegistry::register_event::<TakeDamageEvent>(world);

        world.add_observer(system::apply_effect);
        world.add_observer(system::remove_effect);
        world.add_observer(system::remove_effects);
        // world.add_observer(system::current_stats_update);

        schedule
            .add_systems(event_update_system)
            .add_systems(system::effect_timer_update)
            .add_systems(system::effects_changed_update)
            .add_systems(system::snapshot_ref_count_update);
    }
}

#[godot_api]
impl BattleSystem {
    

    #[func]
    fn set_time_scale(&mut self, time_scale: f64) {
        self.world.resource_mut::<GodotTimeScale>().0 = time_scale;
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

        let (stats, weapon, eq1, eq2, eq3, eq4) = {
            let mut query = self.world.query::<(
                &CurrentStats,
                &Weapon,
                &Equipment1,
                &Equipment2,
                &Equipment3,
                &Equipment4,
            )>();
            query.get(&self.world, origin_entity).unwrap()
        };

        let copied_bundle = EntityBundle {
            current_stats: stats.to_owned(),
            weapon: weapon.clone(),
            equipment1: eq1.clone(),
            equipment2: eq2.clone(),
            equipment3: eq3.clone(),
            equipment4: eq4.clone(),
            ..Default::default()
        };

        let copied_entity = self.world.spawn(copied_bundle).id();
        let id = Uuid::new_v4();
        let mut snapshot_map = self.world.resource_mut::<EntitySnapshotMap>();
        snapshot_map.insert(id, (copied_entity, ref_count));

        Some(id)
    }
}
