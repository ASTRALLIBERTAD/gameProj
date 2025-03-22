use godot::classes::{IResource, Resource, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Resource)]
pub struct Collectibles {
    base: Base<Resource>,

    #[export]
    name: GString,
    #[export]
    icon: Option<Gd<Texture2D>>
}

#[godot_api]
impl IResource for Collectibles {
    fn init(base: Base<Resource>) -> Self {
        Self { 
            base,
            name: GString::new(),
            icon: None
        }
    }

}