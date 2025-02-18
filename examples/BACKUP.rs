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
const GAMMA: f32 = 0.9; // Discount factor for future rewards
const LEARNING_RATE: f32 = 0.1;


struct RaycastPlayerInfo {
    found: bool,
    position: Vector3,
    distance: f32,
}


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
        let mut rng = rand::rng();
        
        self.weights_input_hidden = Array2::from_shape_fn((INPUT_SIZE, HIDDEN_LAYER), 
            |_| rng.random_range(-1.0..1.0));
        self.weights_hidden_output = Array2::from_shape_fn((HIDDEN_LAYER, OUTPUT_SIZE), 
            |_| rng.random_range(-1.0..1.0));
            
        self.max_time = 10.0;
        self.timer = 0.0;
        self.reward = 0.0;
    }

    fn perform_player_raycast(&self, raycast: Gd<RayCast3D>) -> RaycastPlayerInfo {
        // Check if the raycast is colliding with something.
        if raycast.is_colliding() {
            let hit_position = raycast.get_collision_point();
            let agent_position = self.base().get_global_position();
            let distance = (hit_position - agent_position).length();
            // Here you might also check if the collider is the player
            // For example, if let Some(collider) = raycast.get_collider() { ... }
            return RaycastPlayerInfo {
                found: true,
                position: hit_position,
                distance,
            };
        }
        RaycastPlayerInfo {
            found: false,
            position: Vector3::ZERO,
            distance: f32::INFINITY,
        }
    }
    
    #[func]
    fn update_movement(&mut self, raycast: Gd<RayCast3D>, player: Gd<CharacterBody3D>, delta: f64) {
        self.timer += delta as f32;

        if self.timer > self.max_time {
            godot_print!("Failed to reach target! Resetting...");
            self.reward -= 5.0;
            self.reset_agent();
            return;
        }

        // Handle gravity.
        if !self.base_mut().is_on_floor() {
            let gravity = self.base_mut().get_gravity();
            let gravity_vector = Vector3::new(0.0, gravity.y, 0.0) * delta as f32;
            self.base_mut().set_velocity(gravity_vector);
            return;
        }

        let agent_position = self.base_mut().get_global_position();

        // Use raycasting to “find” the player.
        let ray_info = self.perform_player_raycast(raycast);

        // Decide what target position to use.
        let target_position = if ray_info.found {
            ray_info.position
        } else {
            godot_print!("Player not detected by raycast; skipping rotation update.");
            // Fallback: if no target is detected, use the agent's current position.
            // This will be caught by the check below.
            agent_position
        };

        // Check if the target vector (target_position - agent_position) is zero or nearly zero.
        if (target_position - agent_position).length_squared() < 0.0001 {
            godot_print!("Target vector is zero (or nearly zero). Skipping looking_at update.");
        } else {
            // Use looking_at() to update rotation only if the target vector is valid.
            let current_transform = self.base_mut().get_global_transform();
            let new_transform = current_transform.looking_at(target_position, Vector3::new(0.0, 1.0, 0.0), true);
            if new_transform.is_finite() {
                self.base_mut().set_global_transform(new_transform);
            } else {
                godot_print!("Error: Computed transform is not finite, skipping looking_at update.");
            }
        }

        // Use looking_at() (or set_look_at()) to rotate the agent so its front faces the raycast hit point.
        let current_transform = self.base_mut().get_global_transform();
        // Using looking_at() with use_model_front = true:
        let new_transform = current_transform.looking_at(target_position, Vector3::new(0.0, 1.0, 0.0), true);
        if new_transform.is_finite() {
            self.base_mut().set_global_transform(new_transform);
        } else {
            godot_print!("Error: Computed transform is not finite, skipping looking_at update.");
        }

        // Prepare neural network input.
        // Now you can structure the input based on the raycast result rather than the direct player position.
        let input = Array1::from(vec![
            // Instead of player_position.x/y/z, you can use raycast data:
            target_position.x, target_position.y, target_position.z,
            // Include the raycast hit position for additional context.
            ray_info.position.x, ray_info.position.y, ray_info.position.z,
            // For instance, the distance between the agent and the raycast hit.
            (target_position - agent_position).length(),
            agent_position.x, agent_position.y, agent_position.z,
            1.0, // Bias term.
        ]);

        // Get movement from the neural network.
        let movement = self.forward(&input);

        // Create and normalize the movement vector.
        let mut movement_vector = Vector3::new(
            movement[0],
            movement[1].clamp(-VERTICAL_LIMIT, VERTICAL_LIMIT),
            movement[2],
        );
        if movement_vector.length_squared() > MINIMUM_MOVEMENT_THRESHOLD * MINIMUM_MOVEMENT_THRESHOLD {
            movement_vector = movement_vector.normalized() * AGENT_SPEED;
        } else {
            let fallback_direction = target_position - agent_position;
            if fallback_direction.length_squared() > MINIMUM_MOVEMENT_THRESHOLD * MINIMUM_MOVEMENT_THRESHOLD {
                movement_vector = fallback_direction.normalized() * AGENT_SPEED;
            } else {
                movement_vector = Vector3::ZERO;
            }
        }

        // Apply movement.
        self.base_mut().set_velocity(movement_vector);
        self.base_mut().move_and_slide();

        // Calculate reward based on movement.
        let new_distance = (self.base_mut().get_global_position() - target_position).length();
        let old_distance = (agent_position - target_position).length();
        if new_distance < old_distance {
            self.reward += 1.0;
        } else {
            self.reward -= 0.5;
        }

        // Check if reached the target.
        if new_distance < 1.0 {
            godot_print!("Agent reached target! Rewarding...");
            self.reward += 10.0;
            self.reset_agent();
        }

        // Update the neural network using reinforcement learning.
        self.propagate(&input, &movement, self.reward);

        godot_print!("Reward: {}, Timer: {}", self.reward, self.timer);
    }

    

    fn forward(&mut self, input: &Array1<f32>) -> Array1<f32> {
        let hidden = input.dot(&self.weights_input_hidden).mapv(Self::relu);
        let output = hidden.dot(&self.weights_hidden_output);
        output
    }

    fn propagate(&mut self, input: &Array1<f32>, output: &Array1<f32>, reward: f32) {
        let mut rng = rand::rng();
    
        // Compute Q-values (prevent NaN)
        let hidden = input.dot(&self.weights_input_hidden).mapv(Self::relu);
        let q_current = hidden.dot(&self.weights_hidden_output);
    
        // Ensure Q-values are finite
        if !q_current.iter().all(|&x| x.is_finite()) {
            godot_print!("Warning: q_current contains NaN or Inf, skipping update.");
            return;
        }
    
        // Estimate next state (simulate future state with small noise)
        let next_input = input.mapv(|x| x + rng.random_range(-0.1..0.1));
        let next_hidden = next_input.dot(&self.weights_input_hidden).mapv(Self::relu);
        let q_next = next_hidden.dot(&self.weights_hidden_output);
    
        // Ensure q_next is finite
        if !q_next.iter().all(|&x| x.is_finite()) {
            godot_print!("Warning: q_next contains NaN or Inf, skipping update.");
            return;
        }
    
        // Compute Temporal Difference (TD) error with clamping
        let td_error = (reward + GAMMA * q_next.sum() - q_current.sum()).clamp(-10.0, 10.0);
    
        if td_error.is_nan() || td_error.is_infinite() {
            godot_print!("Warning: TD Error is NaN or Inf, skipping update.");
            return;
        }
    
        // Weight updates with TD error
        let delta_hidden_output = hidden.view().insert_axis(ndarray::Axis(1))
            .dot(&output.view().insert_axis(ndarray::Axis(0))) * (LEARNING_RATE * td_error);
        let delta_input_hidden = input.view().insert_axis(ndarray::Axis(1))
            .dot(&hidden.view().insert_axis(ndarray::Axis(0))) * (LEARNING_RATE * td_error);
    
        self.weights_hidden_output += &delta_hidden_output;
        self.weights_input_hidden += &delta_input_hidden;
    
        godot_print!("TD Error: {}", td_error);
    }
    
    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    fn reset_agent(&mut self) {
        godot_print!("Resetting agent...");
    
        // Set a safe position **above** ground to avoid clipping
        let safe_position = Vector3::new(0.0, 1.0, 0.0);
    
        // Create a valid identity transform
        let mut transform = Transform3D::IDENTITY;
        transform.origin = safe_position;
    
        // Set safe transform
        self.base_mut().set_global_transform(transform);
    
        // Reset physics velocity to avoid falling
        self.base_mut().set_velocity(Vector3::ZERO);
    
        // Reset RL variables
        self.timer = 0.0;
        self.reward = 0.0;
    
        godot_print!("Agent reset successfully.");
    }
    
        

    fn face_target(&mut self, target_position: Vector3) {
        let npc_position = self.base_mut().get_global_position();
        let default_up = Vector3::new(0.0, 1.0, 0.0);
        
        // Compute the direction vector from the agent to the target.
        let direction = (target_position - npc_position).normalized();
    
        // Check if the direction is nearly parallel to the default up vector.
        // The dot product will be close to 1 or -1 in that case.
        let alternative_up = if direction.dot(default_up).abs() > 0.99 {
            // If so, choose an alternative up vector (e.g. a horizontal vector).
            // You can choose an arbitrary vector that isn't parallel to direction.
            Vector3::new(0.0, 0.0, 1.0)
        } else {
            default_up
        };
    
        let current_transform = self.base_mut().get_global_transform();
        let new_transform = current_transform.looking_at(target_position, alternative_up, true);
    
        // (Optional) Verify the new transform is valid before applying it.
        if new_transform.is_finite() {
            self.base_mut().set_global_transform(new_transform);
        } else {
            godot_print!("Error: Computed transform is not finite, skipping looking_at update.");
        }
    }

    fn search_for_player(&mut self, delta: f64) {
        godot_print!("Searching for player...");
    
        let rotation_speed = 1.5 * delta as f32; // Adjust speed as needed
        self.base_mut().rotate_y(rotation_speed);
    
        // Optionally, add random movement to explore
        let mut rng = rand::rng();
        let random_direction = Vector3::new(
            rng.random_range(-1.0..1.0),
            0.0,
            rng.random_range(-1.0..1.0)
        ).normalized() * AGENT_SPEED * 0.5; // Move at half speed
    
        self.base_mut().set_velocity(random_direction);
    }
    
    

    
}
