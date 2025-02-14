pub mod damage;
pub mod effect;
pub mod equipment;

#[derive(Debug, strum::EnumString, strum::Display, strum::AsRefStr)]
pub enum GroupTag {
    /// Like arrows, bullets, etc.
    Projectile, // (Projectile)
    /// Like breakable walls, destructible objects, etc.
    Damageable, // (Entity)
    /// Players
    Player, // (Entity)
    /// Enemies
    Enemy, // (Entity)
    /// Walls, unreachable objects, etc.
    Obstacle, // (StaticBody2D)
}
