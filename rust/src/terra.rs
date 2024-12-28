use godot::classes::{ ITileMapLayer, FastNoiseLite, TileMapLayer};
use godot::global::randi;
use godot::obj::NewAlloc;
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
    

    #[export]
    player: Gd<Rustplayer>,
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
            height: 20,
            width: 20,
            loaded_chunks: Vec::new(),
            player: Rustplayer::new_alloc(),
            
    
        }
    }
    fn ready(&mut self) {
        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_seed(randi() as i32);

        self.altitude.set_frequency(0.01);
        
    }

    
    fn process(&mut self, delta: f64) {
        
        let ypo = self.player.get_position();

        let sls = self.base_mut().local_to_map(ypo );
        self.generate_chunk(sls);
        
    }
  
}

#[godot_api]
impl Terrain1 {
  
    #[func]
    fn generate_chunk(&mut self, pos: Vector2i) {
        
        let water = Vector2i::new(0, 11);
        let land = Vector2i::new(1, 0);
        let width = self.width;

        for x in 0..self.width {
            for y in 0..self.height {
                let moist = self.moisture.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let temp = self.temperature.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let alt = self.altitude.get_noise_2d((pos.x - (width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let position = Vector2i::new(pos.x - (width / 2) as i32 + x, pos.y - (self.height / 2) as i32 + y);
                let pos_vec2 = Vector2::new(pos.x as f32, pos.y as f32);
                if !self.loaded_chunks.contains(&pos_vec2) {
                    self.loaded_chunks.push(pos_vec2);
                }
                
                if alt < 0.1 {
                    self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(water)
                    .done();

                } else if alt > 0.0 {
                    self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(land)
                    .done();
                    
                }

                if !self.loaded_chunks.contains(&Vector2 { x: x as f32, y: y as f32 }) {
                    self.loaded_chunks.push(Vector2 { x: x as f32, y: y as f32 });
                }

                
            }
            
        }
    }
    
    
}


