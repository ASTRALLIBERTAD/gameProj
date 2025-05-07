use godot::classes::{ITextureRect, TextureRect};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TextureRect)]
pub struct HeartDisplay {
    base: Base<TextureRect>,

    #[export(range = (0.0, 2.0))]
    pub health: i32,
}

#[godot_api]
impl ITextureRect for HeartDisplay  {
    fn init(base: Base<TextureRect>) -> Self {
        Self { 
            base,
            health: 2, 
        }
    }

}

#[godot_api]
impl HeartDisplay {

}