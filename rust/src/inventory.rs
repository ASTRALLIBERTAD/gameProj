use godot::classes::{IResource, Resource};
use godot::prelude::*;

use crate::item_collectibles::Collectibles;

#[derive(GodotClass)]
#[class(base = Resource)]
pub struct Inventory {
    base: Base<Resource>,

    #[export]
    items: Array<Gd<Collectibles>>,
}

#[godot_api]
impl IResource for Inventory {
    fn init(base: Base<Resource>) -> Self {
        Self { 
            base,
            items: Array::new()
        }
    }

}