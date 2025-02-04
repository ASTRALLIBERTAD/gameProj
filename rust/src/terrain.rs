use godot::classes::{FastNoiseLite, ITileMapLayer, TileMapLayer};
use godot::global::{randi, sqrt};
use godot::obj::WithBaseField;
use godot::prelude::*;
use crate::multiplayer::MultiPlayerRust;
use crate::rustplayer::Rustplayer;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Terrain1 {
    #[base]
    pub base: Base<TileMapLayer>,
    pub moisture: Gd<FastNoiseLite>,
    pub temperature: Gd<FastNoiseLite>,
    pub altitude: Gd<FastNoiseLite>,
    pub height: i32,
    pub width: i32,
    pub loaded_chunks: Array<Vector2i>,

    #[export]
    pub seedser: i32, 
}

#[godot_api]
impl ITileMapLayer for Terrain1 {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            moisture: FastNoiseLite::new_gd(),
            temperature: FastNoiseLite::new_gd(),
            altitude: FastNoiseLite::new_gd(),
            height: 25,
            width: 25,
            loaded_chunks: Array::new(),
            seedser: i32::default(),
        }
    }

    fn ready(&mut self) {
        self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as::<Rustplayer>("/root/main/World/PLAYERS");
        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_frequency(0.01);

    }
    
    fn process(&mut self, _delta: f64) {
        

        self.altitude.set_seed(self.seedser);
        
        let label = self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as::<Rustplayer>("/root/main/World/PLAYERS");
        let ypo = label.get_position();

        let sls = self.base_mut().local_to_map(ypo );
        self.generate_chunk(sls);

        self.unload_distant_chunks( sls);

        let tree = self.base_mut().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let mut multiplayer = tree.get_multiplayer().unwrap();
        let peers = multiplayer.get_peers();
        
        if multiplayer.is_server(){
            for i in peers.to_vec() {
                let pyr = format!("/root/main/World/{}", i);
                let y= root.get_node_as::<MultiPlayerRust>(&pyr);
                    if y.is_instance_valid() {
                        let r = y.get_global_position();
                        let f = self.base_mut().local_to_map(r);
                        self.generate_chunk(f);
                        self.unload_distant_chunks(f);
                        godot_print!("Player {} is valid", i);
                    } else {
                        godot_print!("Player {} is not valid", i);
                }
            }
        }

    }
}

#[godot_api]
impl Terrain1 {  
    
    #[func]
    fn seed_seed(&mut self, seed: i32)-> i32 {
        self.seedser = seed;
        return self.seedser;
    }

    #[func]
    fn tile(&mut self, pid:i32){
        let tree = self.base_mut().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let pyr = format!("/root/main/World/{}", pid);
        let y= root.get_node_as::<MultiPlayerRust>(&pyr);
        if y.is_instance_valid() {
            let r = y.get_global_position();
            let f = self.base_mut().local_to_map(r);
            self.generate_chunk(f);
            self.unload_distant_chunks(f);
            godot_print!("Player klkl {} is valid", pid);
        } else {
                godot_print!("Player klkl {} is not valid", pid);
        }
            

    }
    fn generate_chunk(&mut self, pos: Vector2i) {
        
    
        let water = Vector2i::new(0, 11);
        let land = Vector2i::new(1, 0);

        for x in 0..self.width {
            for y in 0..self.height {
                let _moist = self.moisture.get_noise_2d((pos.x - (self.width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let _temp = self.temperature.get_noise_2d((pos.x - (self.width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let alt = self.altitude.get_noise_2d((pos.x - (self.width / 2) as i32 + x) as f32, (pos.y - (self.height / 2) as i32 + y) as f32) * 10.0;
                let position = Vector2i::new(pos.x - (self.width / 2) as i32 + x, pos.y - (self.height / 2) as i32 + y);
                
                if alt < 0.1 {
                    self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(water)
                    .done();

                } 
                else if alt > 0.0 {
                    self.base_mut().set_cell_ex(position)
                    .source_id(1)
                    .atlas_coords(land)
                    .done();
                }
                if !self.loaded_chunks.contains(Vector2i { x: pos.x as i32, y: pos.y as i32 }) {
                    self.loaded_chunks.push(Vector2i { x: pos.x as i32, y: pos.y as i32 });
                }   
            }
        }
    }

    fn clear_chunk(&mut self, pos: Vector2i) {
        
          
        for x in 0..self.width {

            for y in 0..self.height {
                let width = self.width;
                let height = self.height;
                self.base_mut().set_cell_ex(Vector2i::new(pos.x as i32 - (width / 2) as i32 + x, pos.y as i32- (height / 2) as i32 + y))
                .source_id(-1)
                .atlas_coords(Vector2i::new(-1, -1))
                .alternative_tile(-1)
                .done();
            }
        }
    }

    fn unload_distant_chunks(&mut self, pos: Vector2i) {
        let unload_distance_threshold = (self.width as f32 * 2.0) + 1.0;
        
        for chunk in self.loaded_chunks.iter_shared().collect::<Vec<Vector2i>>() {
            let dist = self.get_dist(chunk, pos);
            if dist > unload_distance_threshold as f64 {
                self.clear_chunk(chunk);
                self.loaded_chunks.erase(chunk);
            }   
        }
    }

    fn get_dist(&mut self, p1: Vector2i, p2: Vector2i) -> f64 {
            let resultant = p1 - p2;
            return sqrt((resultant.x as f64).powi(2) + (resultant.y as f64).powi(2));
    }
    
}