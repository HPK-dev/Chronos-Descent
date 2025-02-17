use bevy_ecs::prelude::*;
use enumset::{EnumSet, EnumSetType};
use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};
use strum::EnumString;
use uuid::Uuid;

use super::damage::DamageTag;

#[derive(Debug, EnumString, strum::Display, EnumSetType)]
pub enum EffectTag {
    // == Stats ==
    /// Restores health over time.
    HealthRegen,
    /// Restores mana over time.
    ManaRegen,

    /// Increase damage dealt.
    DamageBuff,
    /// Increase chance to deal critical damage.
    CritChanceBuff,
    /// Increase damage dealt on critical hits.
    CritDamageBuff,

    /// Reduces incoming damage.
    Resistant,
    /// Absorb incoming damage of specific types.
    Absorb,
    /// Increase max health.
    HealthBoost,
    /// Increase max mana.
    ManaBoost,
    /// Speed up skill cooldowns.
    CooldownReduction,

    /// Increase movement speed.
    Speed,
    /// Decrease movement speed.
    Slow,
    /// Increase attack speed.
    Haste,
    /// Decrease attack speed.
    Cripple,

    /// Damage over time.
    DoT,
    /// Life steal: recover health based on damage dealt.
    LifeSteal,
    /// Mana steal: recover mana based on damage dealt.
    ManaSteal,
    /// Convert part of damage dealt into health.
    LifeConversion,
    /// Damage is absorbed by mana before affecting health.
    ManaShield,
    /// Gain a shield that absorbs damage.
    Shield,
    /// Reflect a percentage of received damage.
    ReflectDamage,
    /// Chance to resist negative effects.
    DebuffResistance,
    /// Chance to evade attacks.
    Evasion,
    /// Completely invulnerable for a duration.
    Invulnerable,
    /// Immune to crowd control effects.
    CCImmunity,
    /// Chance to block an attack entirely.
    Block,

    // == Crowd Control ==
    /// Entity cannot be affected by negative effects and damage.
    Invincible,
    /// Entity immune to specific damage types.
    Immune,
    /// Entity cannot move, attack, or use abilities.
    Stun,
    /// Entity cannot move ; takes damage during the effect.
    Freeze,
    /// Entity cannot move or attack ; takes true damage when effect ends ; when take demage will end the effect.
    DeepFreeze,
    /// Entity cannot heal itself and takes damage over time.
    Burn,
    /// Entity moves slowly and takes damage over time.
    Poison,
    /// Entity cannot attack.
    Disarm,
    /// Entity cannot use abilities.
    Silence,
    /// Entity cannot regenerate mana.
    Drain,
    /// Reduce vision range.
    Blind,
    /// Force entity to move towards the caster.
    Charm,
    /// Force entity to move away from the caster.
    Fear,
    /// Force entity to attack the caster.
    Taunt,
    /// Entity moves slowly and defense is reduced over time.
    Corrosion,
    /// Entity takes damage when casting skills.
    Backlash,
    /// Entity cannot heal.
    HealBlock,
    /// Entity takes additional health damage when using mana.
    AntiMagic,
    /// Entity takes increased damage and deals reduced damage.
    Curse,
    /// Healing effects are reversed into damage.
    ReverseHealing,
    /// Entity is briefly controlled by an enemy.
    MindControl,
    /// Entity loses dodge and block chance.
    Unstable,
    /// Entity size is altered, affecting hitbox and collision.
    SizeChange,
    /// Entity takes damage when effect expires.
    Doom,
    /// Entity takes damage when attacked but effect ends upon hit.
    Daze,
    /// while entity move , lost mana and hp.
    Root,
    /// Entity takes damage every few seconds and spawns enemies nearby.
    Plague,

    /// Allows entity to phase through other entities but remains targetable.
    Ghost,
    /// Entity becomes untargetable (cannot be auto-selected) but remains physically collidable.
    Invisible,

}

#[derive(Debug, Clone)]
pub enum EffectDuration {
    Permanent,
    Temporary(f64),
    Instant,
}

impl From<f64> for EffectDuration {
    fn from(duration: f64) -> Self {
        if duration == 0.0 {
            Self::Instant
        } else if duration < 0.0 {
            Self::Permanent
        } else {
            Self::Temporary(duration)
        }
    }
}

#[derive(Clone)]
pub struct Effect {
    pub kind: EffectTag,
    /// Used for effects that involve damage buff/debuff
    pub damage_tags: EnumSet<DamageTag>,
    pub amount: f64,
    pub duration: EffectDuration,
}

#[derive(Component, Default)]
pub struct Effects(pub FxHashMap<Uuid, Effect>);

impl Deref for Effects {
    type Target = FxHashMap<Uuid, Effect>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Effects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Default)]
pub struct EffectsTimer(pub FxHashMap<Uuid, f64>);

impl Deref for EffectsTimer {
    type Target = FxHashMap<Uuid, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EffectsTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
