use std::str::FromStr;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, Control, ICharacterBody2D, Input, Label};
use godot::prelude::*;

use crate::inventory::Inventory;
use crate::terrain::Terrain1;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct MultiPlayerRust{
    #[base]
    base: Base<CharacterBody2D>,

    #[export]
    sprite: Gd<AnimatedSprite2D>,

    #[export]
    cam: Gd<Camera2D>,

    #[export]
    coords: Gd<Label>,

    #[export]
    invent: Gd<Inventory>,

    #[export]
    item_slot: Gd<Control>,

    is_open: bool
}

#[godot_api]
impl ICharacterBody2D for MultiPlayerRust {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base, 
            sprite: AnimatedSprite2D::new_alloc(),
            cam: Camera2D::new_alloc(),
            coords: Label::new_alloc(),
            invent: Inventory::new_gd(),
            item_slot: Control::new_alloc(),
            is_open: false
        }
    }

    fn enter_tree(&mut self) {
        let r = self.base_mut().get_name().to_int();
        self.base_mut().set_multiplayer_authority( r as i32);
    }

    fn ready(&mut self) {
        if self.base_mut().is_multiplayer_authority(){
            self.cam.make_current();
            self.close();
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
        

        let cord = self.player_cord();
        
        let y_value = if cord.y == 0.0 {
            cord.y * 1.0
        } else {
            cord.y * -1.0
        };

        let k = format!("coordinates :{}, {:?}", cord.x, y_value as i32);
        self.coords.set_text(&k);

        if input.is_action_just_pressed("inventory"){
            if self.is_open{
                self.close();
            }
            else {
                self.open();
            }
        }
    }
}


#[godot_api]
impl MultiPlayerRust {    
    fn player_cord(&mut self) -> Vector2{
        let scene = self.base_mut().get_tree().unwrap().get_root().unwrap().get_node_as::<Terrain1>("/root/main/Terrain/Terrain1");

        let cord = scene.local_to_map(self.base_mut().get_global_position());

        let ko = scene.to_local(Vector2::new(cord.x as f32, cord.y as f32));
        return ko;
    }

    fn open(&mut self){
        self.is_open = true;
        self.item_slot.set_visible(true);
    }

    fn close(&mut self){
        self.is_open = false;
        self.item_slot.set_visible(false);
        
    }
}
