use bevy_ecs::prelude::*;
use uuid::Uuid;

use crate::component::{Damage, Effect};
use godot::obj::InstanceId as GodotInstanceId;

#[derive(Event)]
pub struct RegisterEntityEvent(pub GodotInstanceId);

#[derive(Event)]
pub struct UnregisterEntityEvent(pub GodotInstanceId);

#[derive(Event)]
pub struct TakeDamageEvent(pub GodotInstanceId, pub Damage);

#[derive(Event)]
pub struct RemoveEffectsEvent(pub GodotInstanceId, pub Vec<Uuid>);

#[derive(Event)]
pub struct RemoveEffectEvent(pub GodotInstanceId, pub Uuid);

#[derive(Event)]
pub struct ApplyEffectEvent(pub GodotInstanceId, pub Effect);
