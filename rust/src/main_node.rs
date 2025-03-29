use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainNode {
    base: Base<Node>,
    
}

#[godot_api]
impl INode for MainNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,

        }
    }

    fn ready(&mut self) {

        let scene = load::<PackedScene>("res://world/terrain_1.scn");

        let instance = scene.instantiate_ex().done().unwrap();
        self.base_mut().add_child(&instance);
    }

}
