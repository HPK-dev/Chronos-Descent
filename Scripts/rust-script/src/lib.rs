pub mod entity;
pub mod projectile;
pub mod tag;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
