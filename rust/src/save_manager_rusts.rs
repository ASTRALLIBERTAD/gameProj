
use std::any::Any;

//use std::borrow::Borrow;
//use bincode::serialize;
//use std::fs::File;
//use std::io::prelude::*;
use godot::classes::file_access::ModeFlags;
use godot::classes::{ DirAccess, FileAccess, Node};
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
    player_node_rust: Option<Gd<Rustplayer>>,
    
}

#[godot_api]
impl SaveManagerRust {

    #[func]
    fn save_game_rust(&self, name: String) {
        let base_path = "user://";
        let folder = "games";
        let file_saver = "user://games";
        let name = name;
        let games_path = format!("{}/{}/{}", base_path, folder, name);
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);     
           
        let mut dir = DirAccess::open(base_path).expect("okkk"); 
 
        if !dir.dir_exists(folder) {
                dir.make_dir(folder);
            } 
        
        dir = DirAccess::open(file_saver).expect("not opened");

        if !dir.dir_exists(&name){
            dir.make_dir(&name);
        }

        if dir != DirAccess::open(&games_path).expect("failed to open"){
            return;
        }


   
    }

    #[func]
    fn save_player_pos(&mut self, name: String, players: Gd<Rustplayer>){

        self.player_node_rust = Some(players);
        self.current_world_name = format!("{:?}", name.type_id()).into();
        godot_print!("{}", self.current_world_name);
        let base_path = "user://";
        let folder = "games";
        let name = name;
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);

        let file  = FileAccess::open(&save_path, ModeFlags::WRITE);

        if let Some(mut file) = file {
            if let Some(player) = &self.player_node_rust {
                // Retrieve player position
                let position = player.get_global_position();
                let player_position = PlayerPosition {
                    x: position.x,
                    y: position.y,
                };

                

                // Serialize the position
                match bincode::serialize(&player_position) {
                    Ok(serialized_data) => {
                        // Convert Vec<u8> to PackedByteArray
                        let byte_array = PackedByteArray::from(serialized_data);
                        file.store_buffer(&byte_array);
                        godot_print!("Game saved successfully at {}", save_path);
                    }
                    Err(_) => {
                        godot_error!("Failed to serialize player position");
                    }
                }}}
                else {
                    godot_print!("hi");    
                }
    }

    #[func]
    fn load_player_pos(&mut self, name: String, players: Gd<Rustplayer>) {
        self.player_node_rust = Some(players);

        let base_path = "user://";
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

                    if let Some(player) = self.player_node_rust.as_mut() {
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
                    