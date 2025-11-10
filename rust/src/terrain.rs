use std::collections::{HashMap, HashSet};
use godot::classes::DirAccess;
use godot::classes::{FastNoiseLite, ITileMapLayer, InputEvent, TileMapLayer, file_access::ModeFlags, FileAccess};
use godot::global::randi;
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
    #[inline]
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<V2i> for Vector2i {
    #[inline]
    fn from(v: V2i) -> Self {
        Vector2i::new(v.x, v.y)
    }
}

// Removed SerializableVector2i - use V2i instead for consistency

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

const CHUNK_SIZE: i32 = 16;
const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;
const CHUNK_TILES: usize = CHUNK_SIZE_USIZE * CHUNK_SIZE_USIZE;
const DEFAULT_MAX_CACHED_CHUNKS: usize = 1000;
const LOAD_RADIUS: i32 = 2;
const UNLOAD_DISTANCE: i32 = 4;
const CHUNK_CLEANUP_AGE_SECS: u64 = 600;
const CHUNK_UNLOAD_AGE_SECS: u64 = 300;

impl ChunkData {
    #[inline]
    fn new() -> Self {
        Self {
            tiles: vec![V2i { x: -1, y: -1 }; CHUNK_TILES],
            changed: false,
            last_accessed: std::time::Instant::now(),
        }
    }

    #[inline(always)]
    const fn index(x: usize, y: usize) -> usize {
        y * CHUNK_SIZE_USIZE + x
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, val: V2i) {
        self.tiles[Self::index(x, y)] = val;
        self.changed = true;
        self.last_accessed = std::time::Instant::now();
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> V2i {
        self.tiles[Self::index(x, y)]
    }

    #[inline]
    fn touch(&mut self) {
        self.last_accessed = std::time::Instant::now();
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
    pub loaded_chunks: HashSet<Vector2i>,
    pub path: String,
    #[export]
    pub seedser: i32,
    pub max_cached_chunks: usize,
    pub player_node_names: Array<GString>,
}

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
        let mut main = self.base_mut().get_node_as::<MainNode>("/root/main");
        main.connect("seed_requested", &callable);
    }

    fn ready(&mut self) {
        self.altitude.set_seed(self.seedser);
        self.moisture.set_seed(randi() as i32);
        self.temperature.set_seed(randi() as i32);
        self.altitude.set_frequency(0.01);
        godot_print!("Terrain1 ready with seed: {}", self.seedser);
    }

    fn process(&mut self, _delta: f64) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("click") {
            let k = self.base_mut().get_global_mouse_position();
            let l = self.base_mut().local_to_map(k);
            let coords = Vector2i::new(1, 0);
            
            self.base_mut().set_cell_ex(l)
                .source_id(1)
                .atlas_coords(coords)
                .done();

            let chunk_pos = Self::get_chunk_coord_static(l);
            let entry = self.chunk_cache.entry(chunk_pos).or_insert_with(ChunkData::new);
            
            let lx = l.x.rem_euclid(CHUNK_SIZE) as usize;
            let ly = l.y.rem_euclid(CHUNK_SIZE) as usize;
            entry.set(lx, ly, coords.into());
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
        self.player_positions.insert(id, cord);
        self.generate_chunk_for_player(id, cord);
        self.unload_distant_chunks();
        self.cleanup_old_chunks();
    }

    // Optimized: Pre-allocate and reuse noise map buffer
    fn generate_noise_map(&self, chunk_pos: Vector2i) -> Vec<f32> {
        let mut noise_map = Vec::with_capacity(CHUNK_TILES);
        let start_x = chunk_pos.x * CHUNK_SIZE;
        let start_y = chunk_pos.y * CHUNK_SIZE;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let nx = (start_x + x) as f32;
                let ny = (start_y + y) as f32;
                noise_map.push(self.altitude.get_noise_2d(nx, ny));
            }
        }
        noise_map
    }

    #[inline(always)]
    const fn get_chunk_coord_static(pos: Vector2i) -> Vector2i {
        Vector2i::new(
            pos.x.div_euclid(CHUNK_SIZE),
            pos.y.div_euclid(CHUNK_SIZE),
        )
    }

    #[inline]
    fn get_chunk_coord(&self, pos: Vector2i) -> Vector2i {
        Self::get_chunk_coord_static(pos)
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
        let start_x = chunk_pos.x * CHUNK_SIZE;
        let start_y = chunk_pos.y * CHUNK_SIZE;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let noise = noise_map[ChunkData::index(x as usize, y as usize)];
                let (source_id, coords) = if noise < 0.1 {
                    (1, V2i { x: 0, y: 11 }) // Water
                } else {
                    (2, V2i { x: 1, y: 0 })  // Grass
                };

                chunk.set(x as usize, y as usize, coords);
                let tile_pos = Vector2i::new(start_x + x, start_y + y);
                
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
        
        // Pre-allocate with known size
        let area = ((LOAD_RADIUS * 2 + 1) * (LOAD_RADIUS * 2 + 1)) as usize;
        let mut chunks_to_generate = Vec::with_capacity(area);
        let mut chunks_to_track = Vec::with_capacity(area);

        for dx in -LOAD_RADIUS..=LOAD_RADIUS {
            for dy in -LOAD_RADIUS..=LOAD_RADIUS {
                let chunk_pos = Vector2i::new(center_chunk.x + dx, center_chunk.y + dy);
                chunks_to_track.push(chunk_pos);
                
                if !self.loaded_chunks.contains(&chunk_pos) {
                    chunks_to_generate.push(chunk_pos);
                }
            }
        }

        // Generate chunks
        for chunk_pos in chunks_to_generate {
            self.generate_chunk(chunk_pos);
        }

        // Update player chunk tracking
        let player_chunks = self.player_chunks.entry(player_id).or_insert_with(HashSet::new);
        player_chunks.clear();
        player_chunks.extend(chunks_to_track);
    }

    // Optimized: Check all players at once instead of per-player
    fn unload_distant_chunks(&mut self) {
        let mut chunks_to_unload = Vec::new();

        for &chunk_pos in &self.loaded_chunks {
            let mut keep_chunk = false;

            // Check if any player is close enough
            for &player_pos in self.player_positions.values() {
                let player_chunk_pos = self.get_chunk_coord(player_pos);
                let dx = (chunk_pos.x - player_chunk_pos.x).abs();
                let dy = (chunk_pos.y - player_chunk_pos.y).abs();

                if dx <= UNLOAD_DISTANCE && dy <= UNLOAD_DISTANCE {
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

            // Remove from all player tracking
            for chunks in self.player_chunks.values_mut() {
                chunks.remove(&chunk_pos);
            }

            // Remove from cache if old and unchanged
            if let Some(chunk_data) = self.chunk_cache.get(&chunk_pos) {
                if !chunk_data.changed && chunk_data.last_accessed.elapsed().as_secs() > CHUNK_UNLOAD_AGE_SECS {
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
        let Some(chunk) = self.chunk_cache.get_mut(&chunk_pos) else {
            return;
        };

        if !chunk.changed {
            return;
        }

        // Ensure directory exists
        if !self.path.is_empty() {
            if let Some(mut dir) = DirAccess::open(&self.path) {
                if !dir.dir_exists(".") {
                    dir.make_dir_recursive(".");
                }
            } else if let Some(mut dir) = DirAccess::open("user://") {
                dir.make_dir_recursive(&self.path);
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

    fn load_chunk(&mut self, chunk_pos: Vector2i) -> bool {
        let save_path = format!("{}/chunk_{}_{}.dat", self.path, chunk_pos.x, chunk_pos.y);
        
        let Some(file) = FileAccess::open(&save_path, ModeFlags::READ) else {
            return false;
        };

        let buffer = file.get_buffer(file.get_length() as i64);
        let Ok(decompressed) = decompress_size_prepended(buffer.as_slice()) else {
            return false;
        };

        let Ok(mut chunk) = bincode::deserialize::<ChunkData>(&decompressed) else {
            return false;
        };

        chunk.last_accessed = std::time::Instant::now();
        let start_x = chunk_pos.x * CHUNK_SIZE;
        let start_y = chunk_pos.y * CHUNK_SIZE;

        // Restore tiles
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let coords = chunk.get(x as usize, y as usize);
                if coords.x >= 0 {
                    let pos = Vector2i::new(start_x + x, start_y + y);
                    self.base_mut()
                        .set_cell_ex(pos)
                        .source_id(1)
                        .atlas_coords(coords.into())
                        .done();
                }
            }
        }

        self.chunk_cache.insert(chunk_pos, chunk);
        true
    }

    // Optimized: Use retain instead of building vectors
    fn cleanup_old_chunks(&mut self) {
        if self.chunk_cache.len() <= self.max_cached_chunks {
            return;
        }

        let now = std::time::Instant::now();
        let mut old_chunks: Vec<_> = self.chunk_cache
            .iter()
            .filter(|(&pos, chunk)| {
                !chunk.changed 
                && !self.loaded_chunks.contains(&pos)
                && now.duration_since(chunk.last_accessed).as_secs() > CHUNK_CLEANUP_AGE_SECS
            })
            .map(|(&pos, chunk)| (pos, chunk.last_accessed))
            .collect();

        // Sort by age (oldest first)
        old_chunks.sort_by_key(|(_, time)| *time);

        let remove_count = (self.chunk_cache.len() - self.max_cached_chunks).min(old_chunks.len());
        for (chunk_pos, _) in old_chunks.into_iter().take(remove_count) {
            self.chunk_cache.remove(&chunk_pos);
        }
    }

    fn set_save_path(&mut self, path: String) {
        self.path = path;
    }

    #[inline]
    fn get_loaded_chunk_count(&self) -> i32 {
        self.loaded_chunks.len() as i32
    }

    #[inline]
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
        self.loaded_chunks.remove(&chunk_pos);
        self.chunk_cache.remove(&chunk_pos);
        self.generate_chunk(chunk_pos);
    }

    fn force_save_all_chunks(&mut self) {
        let chunks_to_save: Vec<_> = self.chunk_cache
            .iter()
            .filter(|(_, chunk)| chunk.changed)
            .map(|(&pos, _)| pos)
            .collect();

        for chunk_pos in chunks_to_save {
            self.save_chunk(chunk_pos);
        }
    }
}