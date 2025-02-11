use godot::obj::Gd;

use crate::entity::Entity;

use super::damage::DamageTag;

#[derive(Debug, strum::EnumString, strum::Display)]
pub enum EffectTag {
    // == Stats ==
    /// Restores health over time.
    HealthRegen,
    /// Restores mana over time.
    ManaRegen,

    /// Increase damage dealt.
    DamageBuff(Vec<DamageTag>),
    /// Increase chance to deal critical damage.
    CritChanceBuff(Vec<DamageTag>),
    /// Increase damage dealt on critical hits.
    CritDamageBuff(Vec<DamageTag>),

    /// Reduces incoming damage.
    Resistant(Vec<DamageTag>),
    /// Absorb incoming damage of specific types
    Absorb(Vec<DamageTag>),
    /// Increase max health
    HealthBoost,
    /// Speed up ability cooldowns
    CooldownReduction,

    /// Increase movement speed
    Speed,
    /// Decrease movement speed
    Slow,
    /// Increase attack speed
    Haste,
    /// Decrease attack speed
    Cripple,

    /// Damage over time
    DoT(Vec<DamageTag>),

    // == Crowd Control ==
    /// Entity cannot be affected by negative effects and damage
    Invincible,
    /// Entity immune to specific damage types
    Immune(Vec<DamageTag>),
    /// Entity cannot move, attack, or use abilities
    Stun,
    /// Entity cannot move
    Freeze,
    /// Entity cannot heal itself and take damage for a period of time
    Burn,
    /// Entity move slowly and take damage for a period of time
    Poison,
    /// Entity cannot attack
    Disarm,
    /// Entity cannot use abilities
    Silence,
    /// Entity cannot regenerate health or mana
    Drain,
    ///
    Blind,
    /// Force entity to move towards to the caster
    Charm,
    /// Force entity to move away from the caster
    Fear,
    /// Force entity to attack the caster
    Taunt,
    /// Entity cannot be targeted and can go through other entities
    Ghost,
    /// Entity cannot be targeted
    Invisibile,
}

#[derive(Debug)]
pub enum EffectDuration {
    Permanent,
    Temporary(f64),
    Instant,
}

#[derive(Debug)]
pub struct Effect {
    pub kind: EffectTag,
    pub amount: f64,
    pub duration: EffectDuration,

    pub source: Gd<Entity>,
}
