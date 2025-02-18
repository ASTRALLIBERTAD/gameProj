//this method get nodes from PackedScene using rust

//let scene = load::<PackedScene>("res://Player/players.tscn");
        
//let instance = scene.instantiate().unwrap();
//self.base_mut().add_child(&instance);

use godot::classes::{ ITileMapLayer, Label, TileMapLayer};
use godot::obj::NewGd;
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Tilesm{
    #[base]
    base: Base<TileMapLayer>,

    #[export]
    plays: Gd<PackedScene>
    
}


#[godot_api]
impl ITileMapLayer for Tilesm{

    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            
            base, 
            plays: PackedScene::new_gd(),

        }
    }

    fn ready(&mut self){
        self.op();
        
    }


}

#[godot_api]
impl Tilesm {
    #[func]
    fn op(&mut self) {
        let p: Gd<Node> = self.get_plays().instantiate().unwrap();
        
        
        self.base_mut().add_child(&p);
        let mut label = p.get_node_as::<Label>("/root/Tilesm/PLAYERS/Label");
        
        label.set_text("hello from ptit");

      
    }
    
}