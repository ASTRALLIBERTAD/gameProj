use godot::classes::{IControl, Control};
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=Control)]
pub struct ItemSlot {
    base: Base<Control>,
}

#[godot_api]
impl IControl for ItemSlot {
    fn init(base: Base<Control>) -> Self {
        Self { 
            base,
        }
    }

}