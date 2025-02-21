use crate::{define_mapping, unwrap_or};
use bevy_ecs::prelude::*;
use godot::builtin::Vector2;
use godot::obj::InstanceId;
use rustc_hash::FxHashMap;
use std::fmt::Display;
use uuid::Uuid;

define_mapping! {

    /// Runtime uuid -> remain duration + owner instance id
    #[derive(Default, Resource)]
    EffectsTimer => (FxHashMap<Uuid, (f32, InstanceId)>);

    /// Runtime uuid -> effect data
    #[derive(Default, Component)]
    ModifierEffects => (FxHashMap<Uuid, Vec<StatsModifyEffect>>);

    /// Runtime uuid -> effect data
    #[derive(Default, Component)]
    CrowdControlEffects => (FxHashMap<Uuid, Vec<CrowdControlEffect>>);

    /// Runtime uuid -> effects data
    #[derive(Default, Component)]
    TickEffects => (FxHashMap<Uuid, Vec<TickEffect>>);

    /// Runtime uuid -> effect metadata
    #[derive(Default, Component)]
    EffectsQueue => (FxHashMap<Uuid, EffectMetadata>);
}

impl Display for EffectsQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Effects:
        ======================
        {}
        "#,
            self.0
                .values()
                .map(|v| &v.id)
                .fold(String::new(), |a, b| a + "\n" + b)
        )
    }
}

pub trait GetEffectId {
    fn get_effect_id(&self) -> String;
}

#[derive(Clone)]
/// A bundle of many atomic effects
pub struct Effect {
    /// Effect's unique id. Will be used in i18n.
    pub id: &'static str,
    /// Weather to show this effect on UI
    pub visible: bool,

    /// List of modifier effects
    pub modifier: Vec<(StatsModifyEffect, f32)>,
    /// List of crowd control effects
    pub cc: Vec<(CrowdControlEffect, f32)>,
    /// List of tick effects
    pub tick: Vec<(TickEffect, f32)>,
}

#[derive(Component)]
pub struct EffectMetadata {
    pub id: String,
    pub visible: bool,
}

#[derive(Clone)]
pub struct TickEffect {
    pub kind: TickEffectKind,
    pub interval: f32,
    pub __interval_counter: f32,
}

#[derive(strum::Display, Clone)]
pub enum CrowdControlEffect {
    CannotMove,
    CannotAttack,
    CannotUseSkill,
    CannotBeHealed,
    CannotBeDamaged,
    CannotBeAffectedByPositiveEffect,
    CannotBeAffectedByNegativeEffect,
    CannotBeTargeted,
    RestrictedSight, // This also affects target selector
    DisableCollision,
    DiscardInput,
}

#[derive(strum::Display, Clone)]
pub enum StatsModifyEffect {
    Damage(f32),
    CritChance(f32),
    CritDamage(f32),
    Defense(f32),
    AttackSpeed(f32),
    MovementSpeed(f32),
    MaxHealth(f32),
    MaxMana(f32),
    CooldownReduction(f32),
    Absorption(f32),
}

#[derive(strum::Display, Clone)]
pub enum TickEffectKind {
    HealthRegen(f32),
    ManaRegen(f32),
    PhysicalDamage(f32),
    MagicalDamage(f32),

    DeferredEffect(Effect),
    ForceMove(Vector2),
}

impl GetEffectId for TickEffect {
    fn get_effect_id(&self) -> String {
        self.kind.to_string()
    }
}

impl GetEffectId for CrowdControlEffect {
    fn get_effect_id(&self) -> String {
        self.to_string()
    }
}

impl GetEffectId for StatsModifyEffect {
    fn get_effect_id(&self) -> String {
        self.to_string()
    }
}

macro_rules! define_effect {
    ($(
        $name:ident $([$id:literal])? $(($($arg:ident : $t:ty),+ $(,)?))? {
            $(visible: $visible:expr ;)?
            $(modifier: [ $(
                $mod_kind:ident $(( $($mod_args:expr),+ ))?
            ),* $(,)? ] ;)?
            $(cc: [ $(
                $cc_kind:ident
            ),* $(,)? ] ;)?
            $(tick: [ $(
                $tick_kind:ident $(( $($tick_args:expr),+ ))? { $tick_interval: expr }
            ),* $(,)? ] ;)?
        }
    )*) => { paste::paste!{

        pub enum EffectVariants {
            $([<$name:upper>]  { duration: f32, $($($arg:$t),+)? } ),*
        }

        impl From<EffectVariants> for Effect {
            fn from(ev: EffectVariants) -> Self {
                match ev { $(
                EffectVariants:: [<$name:upper>]{ duration, $($($arg),+)? } => { Effect {
                    id: unwrap_or!(stringify!( [<$name:lower>] ), $($id)? ),
                    visible: unwrap_or!(false, $($visible)?),

                    modifier:
                    ( vec![$( $(
                        (
                            ModifierEffectKind::$mod_kind $(( $($mod_args ),+ ))?,
                            duration
                        )
                    ),* )?] ),

                    cc:
                    ( vec![$( $(
                        ( CrowdControlEffect::$cc_kind, duration)
                    ),* )?] ),

                    tick:
                    vec![ $( $((
                        TickEffect {
                            kind: TickEffectKind::$tick_kind $(( $($tick_args),+ ))?,
                            interval: $tick_interval,
                            __interval_counter: 0.0,
                        },
                        duration
                    )),* )?],

                }}
                )*}
            }
        }
    }};
}

// ============= Define effects =============

define_effect! {
    invincible {
        visible: true;
        cc: [
            CannotBeDamaged,
            CannotBeAffectedByNegativeEffect
        ];
    }

     burn (damage: f32) {
        visible: true;
        cc: [ CannotBeHealed ];
        tick: [
            MagicalDamage(damage){1.0}
        ];
    }

     charm {
        visible: true;
        cc: [
            CannotMove, DiscardInput
        ];
        tick: [
            ForceMove (Vector2 { x: 1.0, y: 1.0 }) {1.0}
        ];
    }
}
