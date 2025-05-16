use std::any::Any;
use std::env::consts::OS;

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

    #[export]
    load_game: GString,
    
}

#[godot_api]
impl SaveManagerRust {

    #[func]
    fn get_os(&self) -> String {
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
        return (&baser).to_string();
    }
    

    #[func]
    pub fn save_game_rust(&self, name: String) {
        let base_path = &self.get_os();
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
    fn save_player_pos(&mut self, name: String){

        self.current_world_name = format!("{:?}", name.type_id()).into();
        godot_print!("Current world name: {}", self.current_world_name);

        // Construct save path
        let base_path = self.get_os();
        let folder = "games";
        let save_path = format!("{}/{}/{}/{}.dat", base_path, folder, name, name);

        // Open the file for writing
        match FileAccess::open(&save_path, ModeFlags::WRITE) {
            Some(mut file) => {
                
                // Retrieve player position
                let position = self.get_player().get_global_position();
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
                
            }
            None => {
                godot_error!("Failed to open save file at {}", save_path);
            }
        }

    }

    #[func]
    fn load_player_pos(&mut self, name: String) {

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

                    
                    self.get_player().set_global_position(Vector2::new(player_position.x, player_position.y));
                    godot_print!("Player position loaded successfully from {}", save_path);
                
                }
                Err(_) => {
                    godot_error!("Failed to deserialize player position from file");
                }
            }
        } else {
            godot_error!("Failed to open file for loading at {}", save_path);
        }
    }


    fn get_player(&mut self) -> Gd<Rustplayer> {
        return self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as::<Rustplayer>("/root/main/World/PLAYERS");
    }

    #[func]
    fn load_game(&mut self, name: GString) {
        self.load_game = name.clone();
        self.load_player_pos(name.to_string());

    }


    #[func]
    fn rust_screenshot(&mut self){
        let world_name = self.load_game.clone();
        self.save_player_pos(world_name.to_string());
        godot_print!("world name is: {}", world_name);
        let path = format!("{}/games/{}/{}.png", self.get_os(), world_name, world_name);
        let screen_capture = self.base_mut().get_viewport().unwrap().get_texture().unwrap().get_image().unwrap();
        screen_capture.save_png(&path);
    }

    #[func]
    fn auto_save(&mut self){
        let world_name = self.load_game.clone();
        godot_print!("world name is: {}", world_name);
        if world_name != "".into() {
            self.save_player_pos(world_name.to_string());
        } else {
            godot_print!("no world");
        }
        

    }
 
}
                    