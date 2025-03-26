use std::ops::DerefMut;

use godot::classes::{IResource, Resource};
use godot::prelude::*;

use crate::inv_slot::InvSlot;
use crate::item_collectibles::Collectibles;

#[derive(GodotClass)]
#[class(base = Resource)]
pub struct Inventory {
    base: Base<Resource>,

    #[export]
    slots: Array<Gd<InvSlot>>,
}

#[godot_api]
impl IResource for Inventory {
    fn init(base: Base<Resource>) -> Self {
        Self { 
            base,
            slots: Array::new()
        }
    }
}

#[godot_api]
impl Inventory {
    #[signal]
    fn update();

    pub fn insert(&mut self, item: Gd<Collectibles>) {
        let mut target_slot: Option<Gd<InvSlot>> = None;

        for mut slot in self.slots.iter_shared()  {
            if slot.bind_mut().get_item() == item {
                target_slot = Some(slot);
                godot_print!("kdkdk");
                break;
            }
            
        }
        if target_slot.is_some(){
            target_slot.unwrap().bind_mut().set_amount(1);
            godot_print!("kkkhhhh");
        }

        else {
            for mut slot in self.slots.iter_shared() {
                if Some(slot.bind_mut().get_item()) == None {
                    target_slot = Some(slot);
                    godot_print!("djh");
                    break;
                }
            }

            if let Some(mut tr_slot) = target_slot{
                tr_slot.bind_mut().set_item(item);
                tr_slot.bind_mut().set_amount(1);
                godot_print!("didd");
            }
        }

        self.base_mut().emit_signal("update", &[]);

    }
}