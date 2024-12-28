
use godot::classes::{ ITileMapLayer, FastNoiseLite, TileMapLayer};
use godot::global::randi;

use godot::prelude::*;

use crate::rustplayer::Rustplayer;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Terrain1 {
    #[base]
    base: Base<TileMapLayer>,
    moisture: Gd<FastNoiseLite>,
    temperature: Gd<FastNoiseLite>,
    altitude: Gd<FastNoiseLite>,
    height: i32,
    width: i32,
    loaded_chunks: Vec<Vector2>,
}

#[godot_api]
impl ITileMapLayer for Terrain1 {
    fn init(base: Base<TileMapLayer>) -> Self {
        godot_print!("Initializing Terrain");
        Self {
            base,
            moisture: FastNoiseLite::new_gd(),
            temperature: FastNoiseLite::new_gd(),
            altitude: FastNoiseLite::new_gd(),
            height: 32,
            width: 32,
            loaded_chunks: Vec::new(),
            
        }
    }

    

    
}

#[godot_api]
impl Terrain1 {
    #[func]
    fn eady(&mut self) {
        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_seed(randi() as i32);

        self.altitude.set_frequency(0.01);
        
    }
    #[func]
    fn ocess(&mut self, tile_map1: Gd<TileMapLayer>, player: Gd<Rustplayer>) {
        let sls = tile_map1.local_to_map(player.get_global_mouse_position());
        self.generate_chunk(sls);
        godot_print!("Player position: {:?}", player);
        
    }
  
    #[func]
    fn generate_chunk(&mut self, pos: Vector2i) {
        godot_print!("Generating chunk at position: {:?}", pos);
        let water = Vector2i::new(0, 0);
        let land = Vector2i::new(1, 0);
        let width = self.width;

        for x in 0..self.width {
            for y in 0..self.height {
                let moist = self.moisture.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let temp = self.temperature.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let alt = self.altitude.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let position = Vector2i::new(pos.x - (width / 2) as i32 + x, pos.y - (self.height / 2) as i32 + y);

                
                if alt < 0.0 {
                    let ok = self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(water)
                    .done();
                    godot_print!("Water: {:?}", ok);

                } else if alt > 0.0 {
                    self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(land)
                    .alternative_tile(0)
                    .done();
                    
                }

                if !self.loaded_chunks.contains(&Vector2 { x: x as f32, y: y as f32 }) {
                    self.loaded_chunks.push(Vector2 { x: x as f32, y: y as f32 });
                }

                
            }
            
        }
    }
    
    
}


