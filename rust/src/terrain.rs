use std::collections::{HashMap, HashSet};
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
    pub player_chunks: HashMap<i32, HashSet<Vector2i>>,
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
            player_chunks: HashMap::new(),
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
        let sls = self.base_mut().local_to_map(ypo);
        
        self.generate_chunk_for_player(0, sls);
        self.unload_distant_chunks_for_player(0, sls);

        let tree = self.base_mut().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let mut multiplayer = tree.get_multiplayer().unwrap();
        let peers = multiplayer.get_peers();
        
        if multiplayer.is_server() {
            for i in peers.to_vec() {
                let pyr = format!("/root/main/World/{}", i);
                let y = root.get_node_as::<MultiPlayerRust>(&pyr);
                if y.is_instance_valid() {
                    let r = y.get_global_position();
                    let f = self.base_mut().local_to_map(r);
                    self.generate_chunk_for_player(i, f);
                    self.unload_distant_chunks_for_player(i, f);
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
    fn seed_seed(&mut self, seed: i32) -> i32 {
        self.seedser = seed;
        self.seedser
    }

    fn generate_chunk_for_player(&mut self, player_id: i32, pos: Vector2i) {
        // Check if chunk is already generated
        if self.player_chunks.get(&player_id).map_or(false, |chunks| chunks.contains(&pos)) {
            return;
        }

        // Generate tile data first
        let mut tiles_to_set = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let position = Vector2i::new(
                    pos.x - (self.width / 2) + x,
                    pos.y - (self.height / 2) + y
                );
                
                let alt = self.altitude.get_noise_2d(
                    position.x as f32,
                    position.y as f32
                ) * 10.0;

                let coords = if alt < 0.1 {
                    Vector2i::new(0, 11)
                } else {
                    Vector2i::new(1, 0)
                };

                tiles_to_set.push((position, coords));
            }
        }

        // Set all tiles
        for (position, coords) in tiles_to_set {
            self.base_mut().set_cell_ex(position)
                .source_id(1)
                .atlas_coords(coords)
                .done();
        }

        // Update player chunks after setting tiles
        self.player_chunks.entry(player_id)
            .or_insert_with(HashSet::new)
            .insert(pos);
    }

    fn unload_distant_chunks_for_player(&mut self, player_id: i32, pos: Vector2i) {
        let unload_distance_threshold = (self.width as f32 * 2.0) + 1.0;
        
        // Collect chunks to unload first
        let chunks_to_unload: Vec<Vector2i> = {
            if let Some(chunks) = self.player_chunks.get(&player_id) {
                chunks.iter()
                    .filter(|&&chunk| {
                        let dist = self.get_dist(chunk, pos);
                        dist > unload_distance_threshold as f64
                    })
                    .cloned()
                    .collect()
            } else {
                Vec::new()
            }
        };

        // Check which chunks are needed by other players
        let mut chunks_to_clear = Vec::new();
        for chunk in &chunks_to_unload {
            let other_players_need_chunk = self.player_chunks.iter()
                .any(|(&pid, player_chunks)| {
                    if pid != player_id && player_chunks.contains(chunk) {
                        // Check if any position in other player's chunks is within range
                        player_chunks.iter().any(|&c| {
                            self.get_dist(c, *chunk) <= unload_distance_threshold as f64
                        })
                    } else {
                        false
                    }
                });

            if !other_players_need_chunk {
                chunks_to_clear.push(*chunk);
            }
        }

        // Clear chunks that are no longer needed
        for chunk in chunks_to_clear {
            self.clear_chunk(chunk);
        }

        // Remove chunks from player's set
        if let Some(chunks) = self.player_chunks.get_mut(&player_id) {
            for chunk in chunks_to_unload {
                chunks.remove(&chunk);
            }
        }
    }

    fn clear_chunk(&mut self, pos: Vector2i) {
        for x in 0..self.width {
            for y in 0..self.height {
                let position = Vector2i::new(
                    pos.x - (self.width / 2) + x,
                    pos.y - (self.height / 2) + y
                );
                self.base_mut().set_cell_ex(position)
                    .source_id(-1)
                    .atlas_coords(Vector2i::new(-1, -1))
                    .alternative_tile(-1)
                    .done();
            }
        }
    }

    fn get_dist(&self, p1: Vector2i, p2: Vector2i) -> f64 {
        let resultant = p1 - p2;
        sqrt((resultant.x as f64).powi(2) + (resultant.y as f64).powi(2))
    }

    #[func]
    fn tile(&mut self, pid: i32) {
        let tree = self.base_mut().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let pyr = format!("/root/main/World/{}", pid);
        let y = root.get_node_as::<MultiPlayerRust>(&pyr);
        if y.is_instance_valid() {
            let r = y.get_global_position();
            let f = self.base_mut().local_to_map(r);
            self.generate_chunk_for_player(pid, f);
            self.unload_distant_chunks_for_player(pid, f);
            godot_print!("Player klkl {} is valid", pid);
        } else {
            godot_print!("Player klkl {} is not valid", pid);
        }
    }
}