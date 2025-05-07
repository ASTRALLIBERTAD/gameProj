mod pet;
mod example;
mod rustplayer;
mod save_manager_rusts;
mod terrain;
mod world;
mod main_node;
mod multiplayer;
mod multiplayer_scene;
mod item_collectibles;
mod inventory;
mod item_slot;
mod inv_slot;
mod heart;
mod heart_display;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}