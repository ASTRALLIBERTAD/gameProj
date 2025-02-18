use std::str;

use godot::classes::{ INode2D, Label, Node2D};
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
    fn ready(&mut self) {
    }

}




