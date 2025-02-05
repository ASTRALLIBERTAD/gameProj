use godot::prelude::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D, RayCast3D};
use rand::Rng;
use ndarray::{Array1, Array2};

const INPUT_SIZE: usize = 11;
const HIDDEN_LAYER: usize = 16;
const OUTPUT_SIZE: usize = 3; // x, y, z movement direction

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Agent {
    #[base]
    base: Base<CharacterBody3D>,
    
    weights_input_hidden: Array2<f32>,
    weights_hidden_output: Array2<f32>,
}

#[godot_api]
impl Agent {
    #[func]
    fn init(&mut self) {
        let mut rng = rand::rng();

        self.weights_input_hidden = Array2::from_shape_fn((INPUT_SIZE, HIDDEN_LAYER), |_| rng.random_range(-1.0..1.0));
        self.weights_hidden_output = Array2::from_shape_fn((HIDDEN_LAYER, OUTPUT_SIZE), |_| rng.random_range(-1.0..1.0));
    }

    #[func]
    fn update_movement(&mut self, raycast: Gd<RayCast3D>, player: Gd<CharacterBody3D>, delta: f64) {
        
        let hit_position = raycast.get_collision_point();
        let npc_position = self.base_mut().get_global_position();

        if !self.base_mut().is_on_floor(){
            let k = self.base_mut().get_gravity();
            self.base_mut().set_velocity(k * delta as f32);
        }

        let input = Array1::from(vec![
            player.get_global_position().x, player.get_global_position().y, player.get_global_position().z,
            hit_position.x, hit_position.y, hit_position.z,
            (player.get_global_position() - hit_position).length(),
            npc_position.x, npc_position.y, npc_position.z,
            1.0 // Bias term
        ]);

        let movement = self.forward(input);
        let movement_vector = Vector3::new(movement[0], movement[1], movement[2]).normalized() * 2.0;

        self.base_mut().set_velocity(movement_vector);
        self.base_mut().move_and_slide();
    }

    fn forward(&self, input: Array1<f32>) -> Array1<f32> {
        let hidden = input.dot(&self.weights_input_hidden).mapv(Self::relu);
        hidden.dot(&self.weights_hidden_output)
    }

    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }
}
