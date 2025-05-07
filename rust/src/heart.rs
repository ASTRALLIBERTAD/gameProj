use godot::classes::{HBoxContainer, IHBoxContainer, Texture2D};
use godot::prelude::*;

use crate::heart_display::{self, HeartDisplay};

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

    heart_list: Vec<Gd<HeartDisplay>>,

    current_health: i32

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
            current_health: 20
        }
    }

    fn ready(&mut self) {
        let heart_parents = self.base_mut().get_children();
        for i in heart_parents.iter_shared() {
            if let Ok(texture_rect) = i.try_cast::<HeartDisplay>() {

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
            if let Ok(mut texture_rect) = i.try_cast::<HeartDisplay>() {
                self.current_health = current_health;

                if texture_rect.bind_mut().get_health() == 2{
                    texture_rect.set_texture(&self.full_heart);
                    godot_print!("full heart");
                }

                else if texture_rect.bind_mut().get_health() == 1 {
                    texture_rect.set_texture(&self.half_heart);
                }
                else if texture_rect.bind_mut().get_health() == 0 {
                    texture_rect.set_texture(&self.empty_heart);
                    
                }

                // for heart_display in &mut self.heart_list {
                //     let texture = if current_health >= 2 {
                //         &self.full_heart
                //     } else if current_health == 1 {
                //         &self.half_heart
                //     } else {
                //         &self.empty_heart
                //     };
        
                //     heart_display.set_texture(texture);
        
                //     // Subtract 2 health points per heart
                //     current_health -= 2;
                // }



                // if health_left >= 2 {
                //     // self.heart_list[0].set_texture(texture);
                //     texture_rect.set_texture(&self.full_heart);
                //     godot_print!("full heart");
                // }
                // else if health_left == 1 {
                //     texture_rect.set_texture(&self.half_heart);
                // }
                // else {
                //     texture_rect.set_texture(&self.empty_heart);
                // }

                self.heart_list.push(texture_rect);
                // Handle the TextureRect here
            }

            
            
        }
        

    }

    pub fn damage(&mut self, damage: i32) {
        let mut current_health = 0;

        let mut remaining_damage = damage;
        let heart_parents = self.base_mut().get_children();
        for (i, heart_display) in heart_parents.iter_shared().enumerate() {
            if let Ok(mut texture_rect) = heart_display.try_cast::<HeartDisplay>() {

                let mut current_heart_health = texture_rect.bind_mut().get_health();
                
                if current_heart_health ==2{
                    let damage_to_apply = remaining_damage.min(2);
                    current_heart_health -= damage_to_apply as i32;
                    texture_rect.bind_mut().set_health(current_heart_health);
                    remaining_damage -= damage_to_apply;
                }

                else if current_heart_health == 1 {
                    let damage_to_apply = remaining_damage.min(1);
                    current_heart_health -= damage_to_apply;
                    texture_rect.bind_mut().set_health(current_heart_health);
                    remaining_damage -= damage_to_apply;
                }

                godot_print!("damage is {:?} and current health {:?}", damage, current_health);
            }

            if remaining_damage <= 0 {
                break;
            }
        }
        current_health -= damage;
        self.update_health(current_health);

        
    }
}