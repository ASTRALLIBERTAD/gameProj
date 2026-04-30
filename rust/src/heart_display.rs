use godot::classes::{ITextureRect, TextureRect};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TextureRect)]
pub struct HeartDisplay {
    base: Base<TextureRect>,

    #[export(range = (0.0, 2.0))]
    #[var(get, set)]
    health: i32,
}

#[godot_api]
impl ITextureRect for HeartDisplay {
    fn init(base: Base<TextureRect>) -> Self {
        Self { base, health: 2 }
    }
}

#[godot_api]
impl HeartDisplay {
    #[func]
    pub fn get_health(&self) -> i32 {
        self.health
    }

    #[func]
    pub fn set_health(&mut self, health: i32) {
        self.health = health
    }
}

