#[derive(Debug, strum::EnumString)]
pub enum DamageTag {
    // == Elements ==
    // Fire,
    // Water,
    // Ice,
    // Thunder,

    // == Damage Types ==
    Physical,
    Magic,
    Real,
    Ability,
    Projectile,
}

#[derive(Debug)]
pub struct Damage {
    pub kind: DamageTag,
    pub amount: f64,

    pub source: String,
}
