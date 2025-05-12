use godot::classes::{HBoxContainer, IHBoxContainer, Texture2D};
use godot::prelude::*;

use crate::heart_display::HeartDisplay;

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

    #[export(range = (0.0, 20.0))]
    pub current_health: i32

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
            current_health: i32::default()
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
        godot_print!("current health is www {:?}", current_health);
        let heart_parents = self.base_mut().get_children();

        if current_health >= 1{
            self.current_health = self.current_health + current_health;
            godot_print!("Hearts: {:?}", self.current_health);
        }

        if current_health <= -1 {
            self.current_health = self.current_health + (current_health);
            godot_print!("Heartsy: {:?}", self.current_health);

        }

        godot_print!("heart current {:?}", self.current_health);

        if self.current_health <=0 {
            self.base_mut().hide();
        }

        for i in heart_parents.iter_shared() {
            if let Ok(mut texture_rect) = i.try_cast::<HeartDisplay>() {

                if texture_rect.bind_mut().get_health() == 2 {
                    texture_rect.set_texture(&self.full_heart);
                    godot_print!("full heart");
                }

                else if texture_rect.bind_mut().get_health() == 1 {
                    texture_rect.set_texture(&self.half_heart);
                }

                else if texture_rect.bind_mut().get_health() == 0 {
                    texture_rect.set_texture(&self.empty_heart);
                    
                }

                self.heart_list.push(texture_rect);

                
                
            }
            
        }
        

    }

    pub fn damage(&mut self, damage: i32) {

        let mut remaining_damage = damage;

        for i in (0..self.heart_list.len()).rev(){
            let heart = &mut self.heart_list[i];
            let mut current_heart_health = heart.bind_mut().get_health();

            if current_heart_health == 2{
                let damage_to_apply = remaining_damage.min(2);
                current_heart_health -= damage_to_apply as i32;
                heart.bind_mut().set_health(current_heart_health);
                remaining_damage -= damage_to_apply;
                godot_print!("damage is {:?} and current health {:?}", damage, self.current_health);
            }

            else if current_heart_health == 1 {
                let damage_to_apply = remaining_damage.min(1);
                current_heart_health -= damage_to_apply;
                heart.bind_mut().set_health(current_heart_health);
                remaining_damage -= damage_to_apply;
                godot_print!("damage is {:?} and current health {:?}", damage, self.current_health);
            }

            

            if remaining_damage <= 0 {
                break;
            }

        }
        self.update_health(-damage);

        
    }

    pub fn heal(&mut self, heal: i32) {
        
        let mut remaining_heal = heal;
        let heart_parents = self.base_mut().get_children();
        for (_i, heart_display) in heart_parents.iter_shared().enumerate() {
            if let Ok(mut texture_rect) = heart_display.try_cast::<HeartDisplay>() {

                let mut current_heart_health = texture_rect.bind_mut().get_health();
                
                if current_heart_health == 0 {
                    let heal_to_apply = remaining_heal.min(2);
                    current_heart_health += heal_to_apply as i32;
                    texture_rect.bind_mut().set_health(current_heart_health);
                    remaining_heal -= heal_to_apply;

                }

                else if current_heart_health == 1 {
                    let heal_to_apply = remaining_heal.min(1);
                    current_heart_health += heal_to_apply;
                    texture_rect.bind_mut().set_health(current_heart_health);
                    remaining_heal -= heal_to_apply;
                }

                godot_print!("heal is {:?}", heal);
            }

            if remaining_heal <= 0 {
                break;
            }
        }


        self.update_health(heal);

        
    }

}