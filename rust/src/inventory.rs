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

    #[func]
    pub fn insert(&mut self, item: Gd<Collectibles>, index1: i32, index2: i32) {

        let item = Some(item);
        
        if index1 < 0{
            for mut slot in self.slots.iter_shared()  {
                if slot.bind_mut().get_item().unwrap().bind_mut().get_stackable() == true{

                    if slot.bind_mut().get_item() == item {
                    
                        let r = slot.bind_mut().get_item().unwrap().bind_mut().get_amount();
                        slot.bind_mut().get_item().unwrap().bind_mut().set_amount(r + 1);
                        self.signals().update().emit();
                        // self.base_mut().emit_signal("update", &[]);
                        godot_print!("Item added to inventory!dldl");
                        return;
                    }
                }

            }
    
            for mut slot in self.slots.iter_shared()  {
    
                if  slot.bind_mut().get_item().unwrap().bind_mut().get_name().is_empty() {
    
                    slot.bind_mut().set_item(item).to_godot();
                    slot.bind_mut().get_item().unwrap().bind_mut().set_amount(1);
                    self.signals().update().emit();
                    // self.base_mut().emit_signal("update", &[]);
                    godot_print!("Item added to inventory!");
                    godot_print!("{:?}", slot.bind_mut().get_item());
                    let r = &self.slots.get(0).unwrap().bind_mut().get_item().unwrap().bind_mut().get_name();
                    godot_print!("kkk {:?}", r);
                    
                    return;                    
                }
                
            }

        }
        else {

            let r = self.slots.get(index1 as usize).unwrap().clone();
            let b =self.slots.get(index2 as usize).unwrap().clone();
            self.slots.set(index1 as usize, &b);
            self.slots.set(index2 as usize, &r);
            self.signals().update().emit();
            // self.base_mut().emit_signal("update", &[]);

            godot_print!("index is not less than 0");
        }

    }
}