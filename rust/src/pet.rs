use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::rustplayer::Rustplayer;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Pet {
    base: Base<CharacterBody2D>,
    is_following: bool,
    speed: f32,
    stop_threshold: f32, 
    follow_distance: f32,  

    #[export]
    player: OnEditor<Gd<Rustplayer>> ,
    #[export]
    sprite: OnEditor<Gd<AnimatedSprite2D>>,

}

#[godot_api]
impl ICharacterBody2D for Pet {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { 
            base,
            is_following: false,
            speed: 100.0,
            stop_threshold: 10.0,
            follow_distance: 100.0,
            player: OnEditor::default(),
            sprite: OnEditor::default(),
        }
    }

    fn process(&mut self, _delta: f64) {
        if !self.player.is_instance_valid() {
            return;
        }
        let player_position = self.player.get_global_position();
        let pet_position = self.base_mut().get_global_position();
        let distance = pet_position.distance_to(player_position);

        if self.is_following{
            if distance > self.stop_threshold {
                self.sprite.play_ex().name("run").done();
                self.move_toward_player();
                self.flip_sprite();
            }
            else {
                self.sprite.play_ex()
                .name("idle")
                .done();
                self.is_following = false;
                self.stop_moving();
            }
        }
        else if distance > self.follow_distance {
            self.is_following = true;
        }
    }
    
}

#[godot_api]
impl Pet {
    fn move_toward_player(&mut self){
        let direction = (self.player.get_global_position() - self.base_mut().get_global_position()).normalized();
        let speed = self.speed;
        self.base_mut().set_velocity((direction)*speed);
        self.base_mut().move_and_slide();
    }

    fn stop_moving(&mut self){
        self.base_mut().set_velocity(Vector2::ZERO);
        self.base_mut().move_and_slide();

    }

    fn flip_sprite(&mut self){
        if self.player.get_global_position().x < self.base_mut().get_global_position().x {
            self.sprite.set_flip_h(true);
        }
        else {
            self.sprite.set_flip_h(false);
        }
    }
    
}
    
