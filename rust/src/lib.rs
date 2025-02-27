mod pet;
mod example;
mod rustplayer;
mod save_manager_rusts;
mod terrain;
mod world;
mod control;
mod main_node;
mod multiplayer;
mod multiplayer_scene;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}