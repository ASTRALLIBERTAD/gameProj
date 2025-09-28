use std::collections::{HashMap, HashSet};
use godot::classes::DirAccess;
use godot::classes::{FastNoiseLite, ITileMapLayer, InputEvent, TileMapLayer, file_access::ModeFlags, FileAccess};
use godot::global::{randi};
use godot::obj::WithBaseField;
use godot::prelude::*;
use serde::{Serialize, Deserialize};
use crate::save_manager_rusts::SaveManagerRust;
use crate::multiplayer::MultiPlayerRust;
use crate::rustplayer::Rustplayer;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vector2i> for SerializableVector2i {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<SerializableVector2i> for Vector2i {
    fn from(s: SerializableVector2i) -> Self {
        Vector2i::new(s.x, s.y)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkData {
    pub tiles: Vec<(SerializableVector2i, SerializableVector2i)>, 
    pub changed: bool,
}

impl ChunkData {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            changed: false,
        }
    }
}

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Terrain1 {
    #[base]
    pub base: Base<TileMapLayer>,
    pub moisture: Gd<FastNoiseLite>,
    pub temperature: Gd<FastNoiseLite>,
    pub altitude: Gd<FastNoiseLite>,
    pub player_chunks: HashMap<i32, HashSet<Vector2i>>,
    pub player_positions: HashMap<i32, Vector2i>,
    pub chunk_cache: HashMap<Vector2i, ChunkData>, // store changed chunks
    #[export]
    pub seedser: i32, 
}

const CHUNK_SIZE: i32 = 16;

#[godot_api]
impl ITileMapLayer for Terrain1 {
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            moisture: FastNoiseLite::new_gd(),
            temperature: FastNoiseLite::new_gd(),
            altitude: FastNoiseLite::new_gd(),
            player_chunks: HashMap::new(),
            player_positions: HashMap::new(),
            chunk_cache: HashMap::new(),
            seedser: i32::default(),
        }
    }

    fn ready(&mut self) {
        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_frequency(0.01);
    }
    
    fn process(&mut self, _delta: f64) {
        self.altitude.set_seed(self.seedser);
        
        // main player
        let label = self.base_mut().get_tree().unwrap().get_root().unwrap()
            .get_node_as::<Rustplayer>("/root/main/World/PLAYERS");
        let ypo = label.get_position();
        let sls = self.base_mut().local_to_map(ypo);
        
        self.player_positions.insert(0, sls);
        self.generate_chunk_for_player(0, sls);
        self.unload_distant_chunks_for_player(0, sls);

        // multiplayer
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
                    self.player_positions.insert(i, f);
                    self.generate_chunk_for_player(i, f);
                    self.unload_distant_chunks_for_player(i, f);
                } else {
                    self.player_positions.remove(&i);
                    self.player_chunks.remove(&i);
                }
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
    if event.is_action_pressed("click") {
        let k = self.base_mut().get_global_mouse_position();
        let l = self.base_mut().local_to_map(k);

        let coords = Vector2i::new(1, 0);

        self.base_mut().set_cell_ex(l)
            .source_id(1)
            .atlas_coords(coords)
            .done();

        // Mark chunk as changed and update stored tiles
        let chunk_pos = self.get_chunk_coord(l);
        let entry = self.chunk_cache.entry(chunk_pos).or_insert_with(ChunkData::new);

        // update or insert tile record
        let mut found = false;
        for (pos, c) in &mut entry.tiles {
            if Vector2i::from(*pos) == l {
                *c = coords.into();
                found = true;
                break;
            }
        }
        if !found {
            entry.tiles.push((l.into(), coords.into()));
        }

        entry.changed = true;
    }
}

}

#[godot_api]
impl Terrain1 {
    fn get_chunk_coord(&self, pos: Vector2i) -> Vector2i {
        Vector2i::new(
            pos.x.div_euclid(CHUNK_SIZE),
            pos.y.div_euclid(CHUNK_SIZE),
        )
    }

    fn generate_chunk(&mut self, chunk_pos: Vector2i) {
    if self.load_chunk(chunk_pos) {
        return; // already loaded from disk
    }

    let mut tiles = Vec::new();
    let start_x = chunk_pos.x * CHUNK_SIZE;
    let start_y = chunk_pos.y * CHUNK_SIZE;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let position = Vector2i::new(start_x + x, start_y + y);

            let alt = self.altitude.get_noise_2d(
                position.x as f32,
                position.y as f32,
            ) * 10.0;

            let coords = if alt < 0.1 {
                Vector2i::new(0, 11) // water
            } else {
                Vector2i::new(1, 0) // grass
            };

            self.base_mut()
                .set_cell_ex(position)
                .source_id(1)
                .atlas_coords(coords)
                .done();

            tiles.push((position.into(), coords.into())); // store tile
        }
    }

    let chunk_data = ChunkData {
        tiles,
        changed: true, // mark as dirty so it will be saved on unload
    };

    self.chunk_cache.insert(chunk_pos, chunk_data);
}



    fn generate_chunk_for_player(&mut self, player_id: i32, pos: Vector2i) {
        let center_chunk = self.get_chunk_coord(pos);
        let load_radius = 1;

        let mut new_chunks = Vec::new();
        if let Some(player_chunks) = self.player_chunks.get(&player_id) {
            for dx in -load_radius..=load_radius {
                for dy in -load_radius..=load_radius {
                    let chunk_pos = Vector2i::new(center_chunk.x + dx, center_chunk.y + dy);
                    if !player_chunks.contains(&chunk_pos) {
                        new_chunks.push(chunk_pos);
                    }
                }
            }
        } else {
            for dx in -load_radius..=load_radius {
                for dy in -load_radius..=load_radius {
                    new_chunks.push(Vector2i::new(center_chunk.x + dx, center_chunk.y + dy));
                }
            }
        }

        for chunk_pos in new_chunks {
            self.generate_chunk(chunk_pos);
            self.player_chunks.entry(player_id).or_insert_with(HashSet::new).insert(chunk_pos);
        }
    }

    fn unload_distant_chunks_for_player(&mut self, player_id: i32, pos: Vector2i) {
        let player_chunk = self.get_chunk_coord(pos);
        let unload_distance = 2;

        let chunks_to_unload: Vec<Vector2i> = self.player_chunks
            .get(&player_id)
            .map(|chunks| {
                chunks.iter()
                    .filter(|&&chunk| {
                        let dx = (chunk.x - player_chunk.x).abs();
                        let dy = (chunk.y - player_chunk.y).abs();
                        dx > unload_distance || dy > unload_distance
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        for chunk in &chunks_to_unload {
            // save before clearing
            self.save_chunk(*chunk);
            self.clear_chunk(*chunk);
        }

        if let Some(chunks) = self.player_chunks.get_mut(&player_id) {
            for chunk in &chunks_to_unload {
                chunks.remove(chunk);
            }
        }
    }


    fn clear_chunk(&mut self, chunk_pos: Vector2i) {
        let start_x = chunk_pos.x * CHUNK_SIZE;
        let start_y = chunk_pos.y * CHUNK_SIZE;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let position = Vector2i::new(start_x + x, start_y + y);

                self.base_mut().set_cell_ex(position)
                    .source_id(-1)
                    .atlas_coords(Vector2i::new(-1, -1))
                    .alternative_tile(-1)
                    .done();
            }
        }
    }

    fn save_chunk(&mut self, chunk_pos: Vector2i) {
    let mut save_manager = self
        .base_mut()
        .get_tree()
        .unwrap()
        .get_root()
        .unwrap()
        .get_node_as::<SaveManagerRust>("/root/RustSaveManager1");

    let load_game = save_manager.bind_mut().load_game.to_string();

    if let Some(chunk) = self.chunk_cache.get(&chunk_pos) {
        if !chunk.changed {
            return;
        }

        // Root save path
        let path = save_manager.bind_mut().get_os(); 
        let base_dir = format!("{}/games/{}/chunk", path, load_game);

        // Ensure directory structure exists
        if let Some(mut dir) = DirAccess::open(&path) {
            if !dir.dir_exists("games") {
                dir.make_dir("games");
            }
            dir.change_dir("games");

            if !dir.dir_exists(&load_game) {
                dir.make_dir(&load_game);
            }
            dir.change_dir(&load_game);

            if !dir.dir_exists("chunk") {
                dir.make_dir("chunk");
            }
        }

        // Final save path
        let save_path = format!("{}/chunk_{}_{}.dat", base_dir, chunk_pos.x, chunk_pos.y);

        // Save file
        if let Some(mut file) = FileAccess::open(&save_path, ModeFlags::WRITE) {
            if let Ok(data) = bincode::serialize(&chunk) {
                let buffer = PackedByteArray::from(data);
                file.store_buffer(&buffer);
                godot_print!("Saved chunk {:?}", chunk_pos);
            }
        }
    }
}


    fn load_chunk(&mut self, chunk_pos: Vector2i) -> bool {
        let mut save_manager = self.base_mut().get_tree().unwrap().get_root().unwrap()
                .get_node_as::<SaveManagerRust>("/root/RustSaveManager1");
        let path = save_manager.bind_mut().get_os();
        let load_game = save_manager.bind_mut().load_game.to_string();
        let save_path = format!("{}//games//{}//chunk//chunk_{}_{}.dat", path, load_game, chunk_pos.x, chunk_pos.y);

    if let Some(file) = FileAccess::open(&save_path, ModeFlags::READ) {
        let buffer = file.get_buffer(file.get_length() as i64);
        if let Ok(chunk) = bincode::deserialize::<ChunkData>(buffer.as_slice()) {
            for (pos, coords) in &chunk.tiles {
                self.base_mut()
                    .set_cell_ex((*pos).into())
                    .source_id(1)
                    .atlas_coords((*coords).into())
                    .done();
            }

            self.chunk_cache.insert(chunk_pos, chunk);
            godot_print!("Loaded chunk {:?}", chunk_pos);
            return true;
        }
    }
    false
}

}
