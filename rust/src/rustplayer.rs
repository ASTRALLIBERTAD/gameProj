use bincode::Options;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, Control, ICharacterBody2D, Input, Label};
use godot::obj::{WithBaseField};
use godot::prelude::*;
use std::str::FromStr;

use crate::heart::Heart;
use crate::inventory::Inventory;
use crate::item_collectibles::Collectibles;

use crate::save_manager_rusts::SaveManagerRust;
use crate::terrain::Terrain1;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Rustplayer {
    #[base]
    base: Base<CharacterBody2D>,

    #[export]
    sprite:  OnEditor<Gd<AnimatedSprite2D>>,

    #[export]
    coords: OnEditor<Gd<Label>>,

    #[export]
    inv: OnEditor<Gd<Inventory>>,

    #[export]
    item_slot: OnEditor<Gd<Control>>,

    is_open: bool,

    #[export]
    heart_ui: OnEditor<Gd<Heart>>,

    #[export]
    pub health: i32,

}

#[godot_api]
impl ICharacterBody2D for Rustplayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            sprite: OnEditor::default(),
            coords: OnEditor::default(),
            inv: OnEditor::default(),
            item_slot: OnEditor::default(),
            is_open: false,
            heart_ui: OnEditor::default(),
            health: i32::default()
        }
    }

    fn ready(&mut self) {

        // self.heart_ui.bind_mut().heal(self.health);
        // godot_print!(" ito {}", self.health);

        // let r = self.base_mut().get_name().to_int();
        // self.base_mut().set_multiplayer_authority(r as i32);
        
        // self.close();
        
    }
    fn process(&mut self, _delta: f64) {
        if !self.base_mut().is_multiplayer_authority() {
            return;
        }

        let speed: f32 = 100.0;

        let input = Input::singleton();

        let direction = Input::get_vector(
            &input,
            &StringName::from_str("left").unwrap(),
            &StringName::from_str("right").unwrap(),
            &StringName::from_str("up").unwrap(),
            &StringName::from_str("down").unwrap(),
        );

        let velocity = direction * speed;

        if input.is_action_just_pressed(&StringName::from_str("left").unwrap()) {
            self.sprite.set_flip_h(true);
            self.heart_ui.bind_mut().heal(1);
        }
        if input.is_action_just_pressed(&StringName::from_str("right").unwrap()) {
            self.sprite.set_flip_h(false);
            self.heart_ui.bind_mut().damage(10);
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

        if input.is_action_just_pressed("inventory") {
            if self.is_open {
                self.close();
            } else {
                self.open();
            }
        }
    }
}

#[godot_api]
impl Rustplayer {
    #[func]
    pub fn tester(&mut self, amount: i32) {
        godot_print!("connected and the amount is : {}", amount);
    }

    fn player_cord(&mut self) -> Vector2 {
        let scene = self
            .base_mut()
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as::<Terrain1>("/root/main/Terrain/Terrain1");

        let cord = scene.local_to_map(self.base_mut().get_global_position());

        let ko = scene.to_local(Vector2::new(cord.x as f32, cord.y as f32));
        return ko;
    }

    fn open(&mut self) {
        self.is_open = true;
        self.item_slot.set_visible(true);
    }

    fn close(&mut self) {
        self.is_open = false;
        self.item_slot.set_visible(false);
    }

    #[func]
    fn collect_items(&mut self, items: Gd<Collectibles>, index: i32) {
        self.inv.bind_mut().insert(items, index, index);
        godot_print!("item index is: {}", index);
        // let y = self.base_mut().emit_signal("fo", &[]);
        // godot_print!("signal: {:?}", y);
        godot_print!("item collected");
    }

    #[func]
    fn open_close(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn player_hp(&mut self, num: i32) {
        self.heart_ui.bind_mut().heal(num);
        self.health = num;
    }

    fn player_health_checker(&mut self) {
        

    }
}
