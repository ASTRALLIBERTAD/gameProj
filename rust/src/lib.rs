mod pet;
mod example;
mod rustplayer;
mod save_manager_rusts;
mod terrain;
mod node2d;
mod control;
mod node;
mod multiplayer;
mod rl;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}