use std::str::FromStr;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct MultiPlayerRust{
    #[base]
    base: Base<CharacterBody2D>,

    #[export]
    sprite: Gd<AnimatedSprite2D>,

    #[export]
    cam: Gd<Camera2D>
}

#[godot_api]
impl ICharacterBody2D for MultiPlayerRust {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base, 
            sprite: AnimatedSprite2D::new_alloc(),
            cam: Camera2D::new_alloc(),
        }
    }

    fn enter_tree(&mut self) {
        let r = self.base_mut().get_name().to_int();
        self.base_mut().set_multiplayer_authority( r as i32);
    }

    fn ready(&mut self) {
        if self.base_mut().is_multiplayer_authority(){
            self.cam.make_current();
        }
    }
    
    fn process(&mut self, _delta:f64){
        if !self.base_mut().is_multiplayer_authority(){
            return;
        }

        let speed: f32 = 100.0;

        let input = Input::singleton();

        let direction = Input::get_vector(&input, &StringName::from_str("left").unwrap(), &StringName::from_str("right").unwrap(), &StringName::from_str("up").unwrap(), &StringName::from_str("down").unwrap());
       
        let velocity = direction * speed;

        if input.is_action_just_pressed( &StringName::from_str("left").unwrap()) {
            self.sprite.set_flip_h(true);
        } 
        if input.is_action_just_pressed( &StringName::from_str("right").unwrap()) {
            self.sprite.set_flip_h(false);
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        
    }
    
}
