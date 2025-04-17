use godot::prelude::*;

use crate::item_collectibles::Collectibles;



#[derive(GodotClass)]
#[class(base=Resource)]
pub struct InvSlot {
    base: Base<Resource>,

    #[export]
    item: Gd<Collectibles>,
}

#[godot_api]
impl IResource for InvSlot {
    fn init(base: Base<Resource>) -> Self {
        Self { 
            base,
            item: Collectibles::new_gd(),
        }
    }

}