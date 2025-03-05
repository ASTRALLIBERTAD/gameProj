use std::fs::DirEntry;

use godot::classes::{Control, DirAccess, IControl, LineEdit};
use godot::prelude::*;

use crate::main_node::MainNode;
use crate::save_manager_rusts::{self, SaveManagerRust};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ControlRust {
    #[base]
    base: Base<Control>,
    #[export]
    world_input: Gd<LineEdit> 


}

#[godot_api]
impl IControl for ControlRust {
    fn init(base: Base<Control>) -> Self {
        Self 
        { 
            base,
            world_input: LineEdit::new_alloc()
        }
    }

    fn ready(&mut self) {
        
    }
}

#[godot_api]
impl ControlRust {

    fn get_saved_games(&mut self){
        if self.base_mut().get_tree().unwrap().get_root().unwrap().has_node("/root/main"){
            self.base_mut().get_node_as::<MainNode>("/root/main").queue_free();
        }

        
        
        //for i in dir.into(){
            
        //}

        self.world_input.set_text("Hello World");

    }

    
}

