use godot::classes::{HBoxContainer, IHBoxContainer, Texture2D, TextureRect};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct Heart {
    base: Base<HBoxContainer>,

    #[export]
    full_heart: Gd<Texture2D>,

    #[export]
    half_heart: Gd<Texture2D>,

    #[export]
    empty_heart: Gd<Texture2D>,

    heart_list: Vec<Gd<TextureRect>>

}

#[godot_api]
impl IHBoxContainer for Heart {
    fn init(base: Base<HBoxContainer>) -> Self {
        Self { 
            base,
            full_heart: Gd::default(),
            half_heart: Gd::default(),
            empty_heart: Gd::default(),
            heart_list: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let heart_parents = self.base_mut().get_children();
        for i in heart_parents.iter_shared() {
            if let Ok(texture_rect) = i.try_cast::<TextureRect>() {

                self.heart_list.push(texture_rect);
                // Handle the TextureRect here
            }
            
        }
        godot_print!("Heart list length: {:?}", self.heart_list);
        godot_print!("Heart UI is ready!");
    }
}

#[godot_api]
impl Heart {

    pub fn update_health(&mut self, current_health: i32){
        godot_print!("uppppdattting health");
        let heart_parents = self.base_mut().get_children();
        for i in heart_parents.iter_shared() {
            if let Ok(mut texture_rect) = i.try_cast::<TextureRect>() {
                let health_left = current_health;
            
                if health_left >= 2 {
                    // self.heart_list[0].set_texture(texture);
                    texture_rect.set_texture(&self.full_heart);
                    godot_print!("full heart");
                }
                else if health_left == 1 {
                    texture_rect.set_texture(&self.half_heart);
                }
                else {
                    texture_rect.set_texture(&self.empty_heart);
                }

                self.heart_list.push(texture_rect);
                // Handle the TextureRect here
            }

            
            
        }
        

    }
}