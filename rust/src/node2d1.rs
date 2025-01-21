use godot::classes::{Control, IControl, LineEdit};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ControlRust {
    #[base]
    base: Base<Control>,
    #[export]
    world_input: Gd<LineEdit> 


}

#[godot_api]
impl IControl for ControlRust {
    fn init(base: Base<Control>) -> Self {
        Self 
        { 
            base,
            world_input: LineEdit::new_alloc()
        }
    }
}

#[godot_api]
impl ControlRust {

    
}

