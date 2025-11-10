use std::collections::{HashMap, HashSet};
use godot::classes::DirAccess;
use godot::classes::{FastNoiseLite, ITileMapLayer, InputEvent, TileMapLayer, file_access::ModeFlags, FileAccess};
use godot::global::{randi};
use godot::obj::WithBaseField;
use godot::prelude::*;
use serde::{Serialize, Deserialize};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

use crate::main_node::MainNode;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vector2i> for V2i {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<V2i> for Vector2i {
    fn from(v: V2i) -> Self {
        Vector2i::new(v.x, v.y)
    }
}

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
    pub tiles: Vec<V2i>,
    #[serde(skip)]
    pub changed: bool,
    #[serde(skip, default = "std::time::Instant::now")]
    pub last_accessed: std::time::Instant,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WorldMeta {
    pub chunks: HashSet<(i32, i32)>,
}

impl Default for ChunkData {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkData {
    fn new() -> Self {
        Self {
            tiles: vec![V2i { x: -1, y: -1 }; CHUNK_SIZE as usize * CHUNK_SIZE as usize],
            changed: false,
            last_accessed: std::time::Instant::now(),
        }
    }

    #[inline]
    fn index(x: usize, y: usize) -> usize {
        y * CHUNK_SIZE as usize + x
    }

    fn set(&mut self, x: usize, y: usize, val: V2i) {
        self.tiles[Self::index(x, y)] = val;
        self.changed = true;
        self.last_accessed = std::time::Instant::now();
    }

    fn get(&self, x: usize, y: usize) -> V2i {
        self.tiles[Self::index(x, y)]
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
    pub chunk_cache: HashMap<Vector2i, ChunkData>,
    pub loaded_chunks: HashSet<Vector2i>, // Track all loaded chunks globally
    pub path: String,
    #[export]
    pub seedser: i32,
    pub max_cached_chunks: usize, // Limit memory usage

    pub player_node_names: Array<GString>,

}

const CHUNK_SIZE: i32 = 16;
const DEFAULT_MAX_CACHED_CHUNKS: usize = 1000;

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
            loaded_chunks: HashSet::new(),
            path: String::default(),
            seedser: i32::default(),
            max_cached_chunks: DEFAULT_MAX_CACHED_CHUNKS,

            player_node_names: Array::default(),

        }
    }

    fn enter_tree(&mut self) {
        let callable = self.base_mut().callable("sync_seed");

        let mut main= self.base_mut().get_node_as::<MainNode>("/root/main");
            
        main.connect("seed_requested", &callable);
        
        
    }

    fn ready(&mut self) {

        // if self.base_mut().is_multiplayer_authority() {
        //     let seed = self.seedser;

        //     self.base_mut().rpc("sync_seed", &[Variant::from(seed)]);
        //     godot_print!("Syncing seed to clients");
        // } else {
        //     godot_print!("Client received seed: {}", self.seedser);
        // }

        self.altitude.set_seed(self.seedser);

        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_frequency(0.01);

        godot_print!("Terrain1 ready with seed: {}", self.seedser);

    }
    
    fn process(&mut self, _delta: f64) {
        
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
            let lx = (l.x.rem_euclid(CHUNK_SIZE)) as usize;
            let ly = (l.y.rem_euclid(CHUNK_SIZE)) as usize;
            entry.set(lx, ly, coords.into());
            entry.changed = true;
        }
    }
}

#[godot_api]
impl Terrain1 {

    pub fn sync_seed(&mut self, seed: i32) {
        self.seedser = seed;
        self.altitude.set_seed(self.seedser);
        godot_print!("Terrain1 synced callable seed: {}", seed);
        
    }


    pub fn update_player_position(&mut self, id: i32, cord: Vector2i) {

        self.player_positions.insert(0, cord);
        self.generate_chunk_for_player(id, cord);
        self.unload_distant_chunks_for_player(id, cord);

        self.cleanup_old_chunks();
    }

    fn generate_noise_map(&self, chunk_pos: Vector2i) -> Vec<f32> {
        let mut noise_map = Vec::with_capacity(CHUNK_SIZE as usize * CHUNK_SIZE as usize);
        let start_x = chunk_pos.x * CHUNK_SIZE as i32;
        let start_y = chunk_pos.y * CHUNK_SIZE as i32;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let nx = (start_x + x as i32) as f32;
                let ny = (start_y + y as i32) as f32;
                noise_map.push(self.altitude.get_noise_2d(nx, ny));
            }
        }

        noise_map
    }

    fn get_chunk_coord(&self, pos: Vector2i) -> Vector2i {
        Vector2i::new(
            pos.x.div_euclid(CHUNK_SIZE),
            pos.y.div_euclid(CHUNK_SIZE),
        )
    }

    fn generate_chunk(&mut self, chunk_pos: Vector2i) {
        if self.loaded_chunks.contains(&chunk_pos) {
            return;
        }

        if self.load_chunk(chunk_pos) { 
            self.loaded_chunks.insert(chunk_pos);
            return; 
        }

        let mut chunk = ChunkData::new();
        let noise_map = self.generate_noise_map(chunk_pos);

        let start_x = chunk_pos.x * CHUNK_SIZE as i32;
        let start_y = chunk_pos.y * CHUNK_SIZE as i32;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let noise = noise_map[ChunkData::index(x as usize, y as usize)];

                let (source_id, coords) = if noise < 0.1 {
                    // Water
                    (1, V2i { x: 0, y: 11 })  
                } else {
                    // Grass
                    (2, V2i { x: 1, y: 0 })   
                };

                chunk.set(x as usize, y as usize, coords);

                let tile_pos = Vector2i::new(start_x + x as i32, start_y + y as i32);
                self.base_mut()
                    .set_cell_ex(tile_pos)
                    .source_id(source_id)  
                    .atlas_coords(coords.into())
                    .done();
            }
        }

        chunk.changed = true;
        self.chunk_cache.insert(chunk_pos, chunk);
        self.loaded_chunks.insert(chunk_pos);
    }


    fn generate_chunk_for_player(&mut self, player_id: i32, pos: Vector2i) {
        let center_chunk = self.get_chunk_coord(pos);
        let load_radius = 2; 

        let mut chunks_to_generate = Vec::new();
        let mut chunks_to_track = Vec::new();
        
        for dx in -load_radius..=load_radius {
            for dy in -load_radius..=load_radius {
                let chunk_pos = Vector2i::new(center_chunk.x + dx, center_chunk.y + dy);
                chunks_to_track.push(chunk_pos);
                
                if !self.loaded_chunks.contains(&chunk_pos) {
                    chunks_to_generate.push(chunk_pos);
                }
            }
        }

        for chunk_pos in chunks_to_generate {
            self.generate_chunk(chunk_pos);
        }

        let player_chunks = self.player_chunks.entry(player_id).or_insert_with(HashSet::new);
        for chunk_pos in chunks_to_track {
            player_chunks.insert(chunk_pos);
        }
    }

    fn unload_distant_chunks_for_player(&mut self, player_id: i32, pos: Vector2i) {
        let player_chunk = self.get_chunk_coord(pos);
        let unload_distance = 4; // Larger unload distance

        // Find chunks that are too far from ANY player
        let mut chunks_to_unload = Vec::new();
        
        for &chunk_pos in &self.loaded_chunks.clone() {
            let mut keep_chunk = false;
            
            // Check if any player is close enough to this chunk
            for (_, &player_pos) in &self.player_positions {
                let player_chunk_pos = self.get_chunk_coord(player_pos);
                let dx = (chunk_pos.x - player_chunk_pos.x).abs();
                let dy = (chunk_pos.y - player_chunk_pos.y).abs();
                
                if dx <= unload_distance && dy <= unload_distance {
                    keep_chunk = true;
                    break;
                }
            }
            
            if !keep_chunk {
                chunks_to_unload.push(chunk_pos);
            }
        }

        // Unload distant chunks
        for chunk_pos in chunks_to_unload {
            self.save_chunk(chunk_pos);
            self.clear_chunk(chunk_pos);
            self.loaded_chunks.remove(&chunk_pos);
            
    
            for (_, chunks) in self.player_chunks.iter_mut() {
                chunks.remove(&chunk_pos);
            }
            
            if let Some(chunk_data) = self.chunk_cache.get(&chunk_pos) {
                if !chunk_data.changed && chunk_data.last_accessed.elapsed().as_secs() > 300 {
                    self.chunk_cache.remove(&chunk_pos);
                }
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

    pub fn save_chunk(&mut self, chunk_pos: Vector2i) {
        if let Some(chunk) = self.chunk_cache.get_mut(&chunk_pos) {
            if !chunk.changed {
                return;
            }

            let dir_path = &self.path;
            if !dir_path.is_empty() {
                if let Some(mut dir) = DirAccess::open(dir_path) {
                    if !dir.dir_exists(".") {
                        dir.make_dir_recursive(".");
                    }
                } else {
                    if let Some(mut dir) = DirAccess::open("user://") {
                        dir.make_dir_recursive(dir_path);
                    }
                }
            }

            let save_path = format!("{}/chunk_{}_{}.dat", self.path, chunk_pos.x, chunk_pos.y);

            if let Some(mut file) = FileAccess::open(&save_path, ModeFlags::WRITE) {
                if let Ok(serialized) = bincode::serialize(&*chunk) {
                    let compressed = compress_prepend_size(&serialized);
                    let buffer = PackedByteArray::from(compressed);
                    file.store_buffer(&buffer);
                    chunk.changed = false;
                }
            }
        }
    }

    fn load_chunk(&mut self, chunk_pos: Vector2i) -> bool {
        let save_path = format!("{}/chunk_{}_{}.dat", self.path, chunk_pos.x, chunk_pos.y);

        if let Some(file) = FileAccess::open(&save_path, ModeFlags::READ) {
            let buffer = file.get_buffer(file.get_length() as i64);
            if let Ok(decompressed) = decompress_size_prepended(buffer.as_slice()) {
                if let Ok(mut chunk) = bincode::deserialize::<ChunkData>(&decompressed) {
                    // Reset the last_accessed time since it was skipped during deserialization
                    chunk.last_accessed = std::time::Instant::now();
                    
                    // Restore tiles
                    for y in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            let coords = chunk.get(x as usize, y as usize);
                            if coords.x >= 0 {
                                let pos = Vector2i::new(
                                    chunk_pos.x * CHUNK_SIZE as i32 + x as i32,
                                    chunk_pos.y * CHUNK_SIZE as i32 + y as i32,
                                );
                                self.base_mut()
                                    .set_cell_ex(pos)
                                    .source_id(1)
                                    .atlas_coords(coords.into())
                                    .done();
                            }
                        }
                    }

                    self.chunk_cache.insert(chunk_pos, chunk);
                    return true;
                }
            }
        }
        false
    }

    // Prevent memory leaks in infinite worlds
    fn cleanup_old_chunks(&mut self) {
        if self.chunk_cache.len() <= self.max_cached_chunks {
            return;
        }

        let mut chunks_to_remove = Vec::new();
        let now = std::time::Instant::now();

        // Find old, unchanged chunks to remove
        for (&pos, chunk) in &self.chunk_cache {
            if !chunk.changed && 
               !self.loaded_chunks.contains(&pos) && 
               now.duration_since(chunk.last_accessed).as_secs() > 600 {
                chunks_to_remove.push(pos);
            }
        }

        // Remove oldest chunks first
        chunks_to_remove.sort_by_key(|&pos| {
            self.chunk_cache.get(&pos).map(|c| c.last_accessed).unwrap_or(now)
        });

        let remove_count = (self.chunk_cache.len() - self.max_cached_chunks).min(chunks_to_remove.len());
        for chunk_pos in chunks_to_remove.into_iter().take(remove_count) {
            self.chunk_cache.remove(&chunk_pos);
        }
    }

    
    fn set_save_path(&mut self, path: String) {
        self.path = path;
    }

    
    fn get_loaded_chunk_count(&self) -> i32 {
        self.loaded_chunks.len() as i32
    }

    
    fn get_cached_chunk_count(&self) -> i32 {
        self.chunk_cache.len() as i32
    }

    
    fn debug_chunk_info(&self, world_pos: Vector2i) -> String {
        let chunk_pos = self.get_chunk_coord(world_pos);
        let is_loaded = self.loaded_chunks.contains(&chunk_pos);
        let is_cached = self.chunk_cache.contains_key(&chunk_pos);
        
        format!(
            "World pos: ({}, {}), Chunk: ({}, {}), Loaded: {}, Cached: {}, Total loaded: {}", 
            world_pos.x, world_pos.y, 
            chunk_pos.x, chunk_pos.y, 
            is_loaded, is_cached, 
            self.loaded_chunks.len()
        )
    }

    
    fn force_generate_chunk_at(&mut self, world_pos: Vector2i) {
        let chunk_pos = self.get_chunk_coord(world_pos);
        // Force regeneration by removing from loaded set
        self.loaded_chunks.remove(&chunk_pos);
        self.chunk_cache.remove(&chunk_pos);
        self.generate_chunk(chunk_pos);
    }

    
    fn force_save_all_chunks(&mut self) {
        let chunks_to_save: Vec<Vector2i> = self.chunk_cache
            .iter()
            .filter(|(_, chunk)| chunk.changed)
            .map(|(&pos, _)| pos)
            .collect();
            
        for chunk_pos in chunks_to_save {
            self.save_chunk(chunk_pos);
        }
    }
}