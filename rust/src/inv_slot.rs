use godot::prelude::*;

use crate::item_collectibles::Collectibles;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct InvSlot {
    base: Base<Resource>,

    #[export]
    #[var(get, set)]
    item: OnEditor<Gd<Collectibles>>,
}

#[godot_api]
impl IResource for InvSlot {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            item: OnEditor::default(),
        }
    }
}
#[godot_api]
impl InvSlot {
    #[func]
    pub fn get_item(&self) -> Gd<Collectibles> {
        let ret = Var::var_pub_get(&self.item);
        // Do something with the value...
        ret
    }

    #[func]
    pub fn set_item(&mut self, item: Gd<Collectibles>) {
        // if let Some(item) = item {
        //     *self.item = item;
        // }
        // if None (null), just do nothing or clear
        Var::var_pub_set(&mut self.item, item);
    }
    #[func]
    pub fn clear_item(&mut self) {
        *self.item = Gd::<Collectibles>::default();
    }
}
