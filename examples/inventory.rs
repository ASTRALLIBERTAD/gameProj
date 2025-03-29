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
        let item_id = item.instance_id();
        let mut modified = false;
        let nil_id = InstanceId::from_i64(0);

        // First pass: look for existing stack
        for mut slot in self.slots.iter_shared() {
            let mut slot_bind = slot.bind_mut();
            let slot_item = slot_bind.get_item();
            
            // Check if slot has valid item and matches our item
            if slot_item.instance_id() != nil_id 
                && slot_item.instance_id() == item_id 
            {
                let current_amount = slot_bind.get_amount();
                slot_bind.set_amount(current_amount + 1);
                modified = true;
                break;
            }
        }

        // Second pass: look for empty slot if not modified
        if !modified {
            for mut slot in self.slots.iter_shared() {
                let mut slot_bind = slot.bind_mut();
                // Check if slot is empty using nil instance ID
                if slot_bind.get_item().instance_id() == nil_id {
                    slot_bind.set_item(item.upcast());
                    slot_bind.set_amount(1);
                    modified = true;
                    break;
                }
            }
        }

        if modified {
            self.base_mut().emit_signal("update", &[]);
        }
    }
}