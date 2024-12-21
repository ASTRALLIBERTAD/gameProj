use godot::classes::{ITileMapLayer, TileMapLayer};
use noise::OpenSimplex;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Terrain {
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for Terrain {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Terrain {

  
   
    
}