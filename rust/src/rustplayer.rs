use std::str::FromStr;
use godot::classes::{  AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input};
use godot::global::sqrt;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Rustplayer{
    #[base]
    base: Base<CharacterBody2D>,

    #[export]
    sprite: Gd<AnimatedSprite2D>,
}

#[godot_api]
impl ICharacterBody2D for Rustplayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base, 
            sprite: AnimatedSprite2D::new_alloc(),
        }
    }
    
    fn process(&mut self, _delta:f64){
        let speed: f32 = 100.0;

        let input = Input::singleton();

        let direction = Input::get_vector(&input, &StringName::from_str("left").unwrap(), &StringName::from_str("right").unwrap(), &StringName::from_str("up").unwrap(), &StringName::from_str("down").unwrap());
       
        let velocity = direction * speed;

        if direction.x == -1.0 {
            self.sprite.set_flip_h(true);
        } 
        if direction.x == 1.0 {
            self.sprite.set_flip_h(false);
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        
    }
    
}
