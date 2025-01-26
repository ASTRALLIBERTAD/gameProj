use godot::classes::{ CharacterBody2D, INode2D, Label, Node2D, TileMapLayer};

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
    opo: Gd<PackedScene>,

    #[export]
    coords: Gd<Label>,

    #[export]
    terras: Gd<PackedScene>,

}

#[godot_api]
impl INode2D for Node2dRust {

    fn init(base: Base<Node2D>) -> Self {
        Self 
        { 
            base,
            players: Rustplayer::new_alloc(),
            coords: Label::new_alloc(),
            terras: PackedScene::new_gd(),
            opo: PackedScene::new_gd(),
        }
    }
    fn ready(&mut self){
        let y: Gd<Rustplayer> = self.get_opo().instantiate_as::<Rustplayer>().upcast();
        self.base_mut().add_child(&y);
        self.fun();
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

    fn fun(&mut self){
        let r = self.terras.instantiate().unwrap();
        self.base_mut().add_child_ex(&r)
        .done();
        godot_print!("hello from rust: {}", r);
    }
    
    fn player_cord(&mut self) -> Vector2{
        let r: Gd<Terrain1> = self.get_terras().instantiate().unwrap().get_parent().upcast().un;
        let cord = r.local_to_map(self.get_players().get_global_position());

        let ko = r.to_local(Vector2::new(cord.x as f32, cord.y as f32));
        return ko;
    }
}