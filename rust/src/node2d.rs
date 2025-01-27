use godot::classes::{ INode2D, Label, Node2D, TileMapLayer};
use godot::obj::NewAlloc;
use godot::prelude::*;

use crate::rustplayer::Rustplayer;
use crate::terrain::Terrain1;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Node2dRust {
    #[base]
    base: Base<Node2D>,

    #[export]
    players: Gd<Rustplayer>,

    #[export]
    coords: Gd<Label>,

}

#[godot_api]
impl INode2D for Node2dRust {
    fn init(base: Base<Node2D>) -> Self {
        Self 
        { 
            base,
            players: Rustplayer::new_alloc(),
            coords: Label::new_alloc(),
        }
    }

    fn physics_process(&mut self, _delta: f64) {

        let cord = self.player_cord();
        
        let y_value = if cord.y == 0.0 {
            cord.y * 1.0
        } else {
            cord.y * -1.0
        };

        let k = format!("coordinates :{}, {:?}", cord.x, y_value as i32);
        self.coords.set_text(&k);

    }

}

#[godot_api]
impl Node2dRust {    
    fn player_cord(&mut self) -> Vector2{
        let scene = self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as::<Terrain1>("/root/main/Terrain/Terrain1");

        let cord = scene.local_to_map(self.get_players().get_global_position());

        let ko = scene.to_local(Vector2::new(cord.x as f32, cord.y as f32));
        return ko;
    }
}