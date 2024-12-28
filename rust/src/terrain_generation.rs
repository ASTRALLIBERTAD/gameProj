use std::ops::BitAndAssign;

use godot::classes::ITileMapLayer;
use godot::obj::NewAlloc;
use godot::prelude::*;
use godot::classes::{FastNoiseLite, TileMapLayer};

use crate::rustplayer::Rustplayer;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Terrain {
    #[base]
    base: Base<TileMapLayer>,
    moisture: Gd<FastNoiseLite>,
    temperature: Gd<FastNoiseLite>,
    altitude: Gd<FastNoiseLite>,
    player_node_rust: Option<Gd<Rustplayer>>,
    player: Gd<Rustplayer>,
    height: i32,
    width: i32,
    loaded_chunks: Vec<Vector2>,
}

#[godot_api]
impl ITileMapLayer for Terrain {
    fn init(base: Base<TileMapLayer>) -> Self {
        godot_print!("Initializing Terrain");
        Self {
            base,
            moisture: FastNoiseLite::new_gd(),
            temperature: FastNoiseLite::new_gd(),
            altitude: FastNoiseLite::new_gd(),
            player: Rustplayer::new_alloc(),
            height: 32,
            width: 32,
            loaded_chunks: Vec::new(),
            player_node_rust: Some(Rustplayer::new_alloc()),
        }
    }
}

#[godot_api]
impl Terrain {
    
    #[func]
    fn generate_tiles(&mut self) {
        godot_print!("Generating tiles");
        let player_position = self.player.get_position();
        godot_print!("Player position: {:?}", player_position);
        let player_tile_pos = self.base_mut().local_to_map(player_position);
        godot_print!("Player tile position: {:?}", player_tile_pos);
        
        self.generate_chunk(Vector2::new(player_tile_pos.x as f32, player_tile_pos.y as f32));
        self.unload_distant_chunks(Vector2::new(player_tile_pos.x as f32, player_tile_pos.y as f32));
    }
    #[func]
    fn generate_chunk(&mut self, pos: Vector2) {
        godot_print!("Generating chunk at position: {:?}", pos);
        let water = Vector2i::new(0, 0);
        let width = self.width;
        let height = self.height;
        for x in 0..width {
            for y in 0..height {
                let moist = self.moisture.get_noise_2d(pos.x - (width / 2) as f32 + x as f32, pos.y - (height / 2) as f32 + y as f32) * 10.0;
                let temp = self.temperature.get_noise_2d(pos.x - (width / 2) as f32 + x as f32, pos.y - (height / 2) as f32 + y as f32) * 10.0;
                let alt = self.altitude.get_noise_2d(pos.x - (width / 2) as f32 + x as f32, pos.y - (height / 2) as f32 + y as f32) * 10.0;
                let position = Vector2i::new((pos.x - (width / 2) as f32 + x as f32) as i32, (pos.y - (height / 2) as f32 + y as f32) as i32);

                godot_print!("moist: {}, temp: {}, alt: {}", moist, temp, alt);
                godot_print!("Setting cell at position: {:?}", position);

                if alt < 0.0 {
                    self.base_mut().set_cell(water);
                    godot_print!("Set water cell at position: {:?}", water);
                } else {
                    self.base_mut().set_cell(water);
                    godot_print!("Set terrain cell at position: {:?}", water);
                }

                if !self.loaded_chunks.contains(&Vector2::new(pos.x, pos.y)) {
                    self.loaded_chunks.push(Vector2::new(pos.x, pos.y));
                }
            }
        }
    }

    fn unload_distant_chunks(&mut self, player_pos: Vector2) {
        godot_print!("Unloading distant chunks");
        let unload_distance_threshold = (self.width * 2) + 1;

        let mut chunks_to_remove = Vec::new();
        for &chunk in self.loaded_chunks.clone().iter() {
            let distance_to_player = self.get_dist(chunk, player_pos);
            if distance_to_player > unload_distance_threshold as f32 {
                self.clear_chunk(chunk);
                chunks_to_remove.push(chunk);
            }
        }
        self.loaded_chunks.retain(|chunk| !chunks_to_remove.contains(chunk));
    }

    fn clear_chunk(&mut self, pos: Vector2) {
        godot_print!("Clearing chunk at position: {:?}", pos);
        let width = self.width;
        let height = self.height;
        for x in 0..width {
            for y in 0..height {
                self.base_mut().set_cell(Vector2i::new((pos.x - (width / 2) as f32 + x as f32) as i32, (pos.y - (height / 2) as f32 + y as f32) as i32));
            }
        }
    }

    fn get_dist(&self, p1: Vector2, p2: Vector2) -> f32 {
        let resultant = p1 - p2;
        (resultant.x.powi(2) + resultant.y.powi(2)).sqrt()
    }
}