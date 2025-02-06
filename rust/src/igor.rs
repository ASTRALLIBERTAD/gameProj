use godot::prelude::*;
use godot::classes::{CharacterBody3D, RayCast3D};
use rand::Rng;
use ndarray::{Array1, Array2};

const INPUT_SIZE: usize = 11;
const HIDDEN_LAYER: usize = 16;
const OUTPUT_SIZE: usize = 3;
const MINIMUM_MOVEMENT_THRESHOLD: f32 = 0.001;
const AGENT_SPEED: f32 = 2.0;
const VERTICAL_LIMIT: f32 = 0.5;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Agent {
    #[base]
    base: Base<CharacterBody3D>,
    
    weights_input_hidden: Array2<f32>,
    weights_hidden_output: Array2<f32>,
    timer: f32,
    max_time: f32,
    reward: f32,
}

#[godot_api]
impl Agent {
    #[func]
    fn init(&mut self) {
        let mut rng = rand::rng(); // Fixed: Use thread_rng() instead of rng()
        
        self.weights_input_hidden = Array2::from_shape_fn((INPUT_SIZE, HIDDEN_LAYER), 
            |_| rng.random_range(-1.0..1.0)); // Fixed: Use gen_range instead of random_range
        self.weights_hidden_output = Array2::from_shape_fn((HIDDEN_LAYER, OUTPUT_SIZE), 
            |_| rng.random_range(-1.0..1.0));
            
        self.max_time = 10.0;
        self.timer = 0.0;
        self.reward = 0.0;
    }

    #[func]
    fn update_movement(&mut self, raycast: Gd<RayCast3D>, player: Gd<CharacterBody3D>, delta: f64) {
        self.timer += delta as f32;

        if self.timer > self.max_time {
            godot_print!("Failed to reach player! Resetting...");
            self.reward -= 5.0;
            self.reset_agent();
            return;
        }

        // Handle gravity
        if !self.base_mut().is_on_floor() {
            let gravity = self.base_mut().get_gravity();
            let gravity_vector = Vector3::new(0.0, gravity.y, 0.0) * delta as f32;
            self.base_mut().set_velocity(gravity_vector);
            return; // Don't process movement while falling
        }

        let npc_position = self.base_mut().get_global_position();
        let player_position = player.get_global_position();
        let hit_position = raycast.get_collision_point();
        
        // Calculate distance to player before movement
        let distance_before = (npc_position - player_position).length();

        // Prepare neural network input
        let input = Array1::from(vec![
            player_position.x, player_position.y, player_position.z,
            hit_position.x, hit_position.y, hit_position.z,
            (player_position - hit_position).length(),
            npc_position.x, npc_position.y, npc_position.z,
            1.0 // Bias term
        ]);

        // Get movement from neural network
        let movement = self.forward(input);
        
        // Create movement vector with proper safety checks
        let movement_vector = Vector3::new(
            movement[0],
            movement[1].clamp(-VERTICAL_LIMIT, VERTICAL_LIMIT),
            movement[2]
        );

        // Safety check for zero vector
        let movement_vector = if movement_vector.length_squared() > MINIMUM_MOVEMENT_THRESHOLD * MINIMUM_MOVEMENT_THRESHOLD {
            movement_vector.normalized() * AGENT_SPEED
        } else {
            // If movement is too small, move directly towards player
            let direction = player_position - npc_position;
            if direction.length_squared() > MINIMUM_MOVEMENT_THRESHOLD * MINIMUM_MOVEMENT_THRESHOLD {
                direction.normalized() * AGENT_SPEED
            } else {
                Vector3::ZERO // Stay still if too close
            }
        };

        // Apply movement
        self.base_mut().set_velocity(movement_vector);
        self.base_mut().move_and_slide();

        // Calculate reward based on new position
        let distance_after = (self.base_mut().get_global_position() - player_position).length();
        
        if distance_after < distance_before {
            self.reward += 1.0;
        } else {
            self.reward -= 0.5;
        }

        // Check if reached player
        if distance_after < 1.0 {
            godot_print!("Agent reached player! Rewarding...");
            self.reward += 10.0;
            self.reset_agent();
        }

        godot_print!("Reward: {}, Timer: {}", self.reward, self.timer);
    }

    fn forward(&mut self, input: Array1<f32>) -> Array1<f32> {
        let hidden = input.dot(&self.weights_input_hidden).mapv(Self::relu);
        let output = hidden.dot(&self.weights_hidden_output);
    
        let learning_rate = 0.1;
        let reward_signal = self.reward / 10.0;
    
        // Use broadcasting for weight updates
        let delta_input_hidden = input.view().insert_axis(ndarray::Axis(1))
            .dot(&hidden.view().insert_axis(ndarray::Axis(0)));
        let delta_hidden_output = hidden.view().insert_axis(ndarray::Axis(1))
            .dot(&output.view().insert_axis(ndarray::Axis(0)));
    
        self.weights_input_hidden += &(delta_input_hidden * (learning_rate * reward_signal));
        self.weights_hidden_output += &(delta_hidden_output * (learning_rate * reward_signal));
    
        output
    }

    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    fn reset_agent(&mut self) {
        self.base_mut().set_global_position(Vector3::new(0.0, 1.0, 0.0));
        self.timer = 0.0;
        self.reward = 0.0;
    }
}