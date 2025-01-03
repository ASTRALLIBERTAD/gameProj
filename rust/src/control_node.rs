use godot::classes::{INode2D, Label, Node2D, TileMapLayer, Timer};
use godot::obj::NewAlloc;
use godot::prelude::*;

use crate::rustplayer::Rustplayer;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Node2dRust {
    #[base]
    base: Base<Node2D>,

    #[export]
    players: Gd<Rustplayer>,
    #[export]
    tile: Gd<TileMapLayer>,
    
}

#[godot_api]
impl INode2D for Node2dRust {
    fn init(base: Base<Node2D>) -> Self {
        Self 
        { 
            base,
            players: Rustplayer::new_alloc(),
            tile: TileMapLayer::new_alloc(),
        }
    }

    fn ready(&mut self){
        let mut r =self.base_mut().get_node_as::<Timer>("/root/World/AutoSave");
        r.set_autostart(true);
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut r =self.base_mut().get_node_as::<Label>("/root/Node2dRust/CanvasLayer/Label");
        let y = self.player_cord();
        let k = format!("{}, {}", y.x, y.y);
        r.set_text(&k);
        

    }
}

#[godot_api]
impl Node2dRust {

    fn player_cord(&mut self) -> Vector2{
        //let tile = self.base_mut().get_node_as::<TileMapLayer>("/root/Node2dRust/TileMapLayer");
        let tile = self.tile.clone();
        let cord = tile.local_to_map(self.get_players().get_global_position());
        let ko = tile.to_local(Vector2::new(cord.x as f32, cord.y as f32));
        return ko;
    }
    
}