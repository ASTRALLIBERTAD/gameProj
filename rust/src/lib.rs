mod player;
mod rustplayer;
mod save_manager_rusts;
mod terra;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}