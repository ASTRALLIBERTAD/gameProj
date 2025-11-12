use godot::classes::{INode, Node};
use godot::prelude::*;

use crate::terrain::Terrain1;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainNode {
    base: Base<Node>,
    #[export]
    terrain_scene: OnEditor<Gd<PackedScene>>,
}

#[godot_api]
impl INode for MainNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            terrain_scene: OnEditor::default(),

        }

        
    }

    fn ready(&mut self) {
    }

}

#[godot_api]
impl MainNode {

    #[signal]
    fn seed_requested(seed: i32);

    #[func]
    #[rpc(authority, call_remote, reliable)]
    fn seed(&mut self, seed: i32) {

        self.signals().seed_requested().emit(seed);

        if let Some(mut terrain) = self.base_mut().try_get_node_as::<Terrain1>("/root/main/Terrain/Terrain1") {
            
            terrain.bind_mut().sync_seed(seed);
            

            godot_print!("This is the seed: {}", seed);
        } else {
            godot_print!("Could not find Terrain1 node");
        }
    }
    
}
