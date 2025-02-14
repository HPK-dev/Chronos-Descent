use enumset::{EnumSet, EnumSetType};
use strum::EnumString;

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
    /// Absorb incoming damage of specific types
    Absorb,
    /// Increase max health
    HealthBoost,
    /// Speed up skill cooldowns
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
    DoT,

    // == Crowd Control ==
    /// Entity cannot be affected by negative effects and damage
    Invincible,
    /// Entity immune to specific damage types
    Immune,
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

#[derive(Debug)]
pub struct Effect {
    pub kind: EffectTag,
    /// Used for effects that involve damage buff/debuff
    pub damage_tags: EnumSet<DamageTag>,
    pub amount: f64,
    pub duration: EffectDuration,
}

impl Effect {
    pub fn new(
        kind: EffectTag,
        amount: f64,
        duration: f64,
        damage_tags: EnumSet<DamageTag>,
    ) -> Self {
        Self {
            kind,
            damage_tags,
            amount,
            duration: duration.into(),
        }
    }
}
