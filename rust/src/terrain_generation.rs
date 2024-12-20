use godot::classes::{ITileMap, TileMap};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMap)]
pub struct Terrain {
    base: Base<TileMap>,
}

#[godot_api]
impl ITileMap for Terrain {
    fn init(base: Base<TileMap>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Terrain {

    #[func]
    fn generate_tiles() {

    }
    
}