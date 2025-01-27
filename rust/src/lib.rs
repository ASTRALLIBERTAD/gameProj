mod player;
mod rustplayer;
mod save_manager_rusts;
mod terrain;
mod node2d;
mod control;
mod trial;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}