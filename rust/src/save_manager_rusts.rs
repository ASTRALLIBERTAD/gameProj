use std::any::Any;
use std::env::consts::OS;

//use std::borrow::Borrow;
//use bincode::serialize;
//use std::fs::File;
//use std::io::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::classes::{ DirAccess, FileAccess, Node, TileMapLayer};
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use crate::rustplayer::Rustplayer;


#[derive(Serialize, Deserialize)]
struct PlayerPosition {
    x: f32,
    y: f32,
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct SaveManagerRust {
  
    #[base]
    base: Base<Node>,
    current_world_name: StringName,
    tile_for_player: Option<Gd<TileMapLayer>>,
    
}

#[godot_api]
impl SaveManagerRust {

    fn get_os(&self) -> &str {
        let mut baser: &str = "";
        if OS == "windows" {
            baser = "user://";
            godot_print!("windows");
        }
        if OS == "android" {
            baser = "/storage/emulated/0/Android/data/com.example.proj/files/";
            godot_print!("android");  
        }
        godot_print!("{}", baser);
        return &baser;
    }
    

    #[func]
    fn save_game_rust(&self, name: String) {
        let base_path = self.get_os();
        let folder = "games";
        let file_saver = format!("{}/{}", base_path, folder);
        let name = name;
        let games_path = format!("{}/{}/{}", base_path, folder, name);        
           
        let mut dir = DirAccess::open(base_path).expect("ok"); 
 
        if !dir.dir_exists(folder) {
                dir.make_dir(folder);
        } 
        dir = DirAccess::open(&file_saver).expect("not opened");

        if !dir.dir_exists(&name){
            dir.make_dir(&name);
        }
        if dir != DirAccess::open(&games_path).expect("failed to open"){
            return;
        }
    }

    #[func]
    fn save_player_pos(&mut self, name: String, players: Option<Gd<Rustplayer>>){

        self.current_world_name = format!("{:?}", name.type_id()).into();
        godot_print!("Current world name: {}", self.current_world_name);

        // Construct save path
        let base_path = self.get_os();
        let folder = "games";
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);

        // Open the file for writing
        match FileAccess::open(&save_path, ModeFlags::WRITE) {
            Some(mut file) => {
                if let Some(player) = players {
                    // Retrieve player position
                    let position = player.get_global_position();
                    let player_position = PlayerPosition {
                        x: position.x,
                        y: position.y,
                    };

                    // Serialize the position with a size limit
                    match bincode::serialize(&player_position) {
                        Ok(serialized_data) => {
                            if serialized_data.len() <= 4096 { // Example size limit (4KB)
                                let byte_array = PackedByteArray::from(serialized_data);
                                file.store_buffer(&byte_array);
                                godot_print!("Game saved successfully at {}", save_path);
                            } else {
                                godot_error!("Serialized data exceeds the size limit!");
                            }
                        }
                        Err(e) => {
                            godot_error!("Failed to serialize player position: {}", e);
                        }
                    }
                } else {
                    godot_error!("Player instance is missing!");
                }
            }
            None => {
                godot_error!("Failed to open save file at {}", save_path);
            }
        }

    }

    #[func]
    fn load_player_pos(&mut self, name: String, players: Option<Gd<Rustplayer>>) {

        let base_path = self.get_os();
        let folder = "games";
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);

        // Open the file for reading
        let file = FileAccess::open(&save_path, ModeFlags::READ);

        // Check if the file was successfully opened
        if let Some(file) = file {
            // Read the data from the file into a buffer
            let data = file.get_buffer(file.get_length() as i64);

            // Convert PackedByteArray to &[u8] for deserialization
            let data_slice: &[u8] = data.as_slice();

            // Deserialize the player position data
            match bincode::deserialize::<PlayerPosition>(data_slice) {
                Ok(player_position) => {

                    if let Some(mut player) = players {
                        player.set_global_position(Vector2::new(player_position.x, player_position.y));
                        godot_print!("Player position loaded successfully from {}", save_path);
                    }
                }
                Err(_) => {
                    godot_error!("Failed to deserialize player position from file");
                }
            }
        } else {
            godot_error!("Failed to open file for loading at {}", save_path);
        }
    }
}
                    