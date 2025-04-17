use godot::classes::{IResource, Resource, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Resource)]
pub struct Collectibles {
    base: Base<Resource>,

    #[export]
    name: GString,
    #[export]
    amount: i32,
    #[export]
    icon: Option<Gd<Texture2D>>,
    #[export]
    stackable: bool,
}

#[godot_api]
impl IResource for Collectibles {
    fn init(base: Base<Resource>) -> Self {
        Self { 
            base,
            name: GString::default(),
            amount: i32::default(),
            icon: None,
            stackable: bool::default(),

        }
    }

}