use godot::prelude::*;
use godot::classes::{CharacterBody3D, RayCast3D, Node3D};
use rand::Rng;
use ndarray::{Array1, Array2, arr2};

const INPUT_SIZE: usize = 11;
const HIDDEN_LAYER: usize = 32;
const OUTPUT_SIZE: usize = 2;
const AGENT_SPEED: f32 = 3.5;
const RAYCAST_NUM: usize = 7;
const MAX_DETECTION_RANGE: f32 = 30.0;
const FOV: f32 = 120.0* std::f32::consts::PI / 180.0;

// Reward system
const REWARD_FOUND_PLAYER: f32 = 5.0;
const REWARD_CLOSER: f32 = 1.5;
const PUNISH_LOST_PLAYER: f32 = -3.0;
const PUNISH_TIMEOUT: f32 = -5.0;
const REWARD_MAINTAIN_LOS: f32 = 0.2;
const PUNISH_STUCK: f32 = -2.0;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Agent {
    #[base]
    base: Base<CharacterBody3D>,
    
    // Neural Network
    weights_input_hidden: Array2<f32>,
    weights_hidden_output: Array2<f32>,
    
    // Agent state
    raycasts: Vec<Gd<RayCast3D>>,
    last_player_pos: Vector3,
    player_visible: bool,
    last_distance: f32,
    episode_reward: f32,
    positions_history: Vec<Vector3>,
    rotation_angle: f32,
    velocity_history: Vec<Vector3>,
}

#[godot_api]
impl Agent {

    fn velocity_history_x(&self) -> f32 {
        self.velocity_history.iter()
            .map(|v| v.x)
            .sum::<f32>() / self.velocity_history.len() as f32
    }

    fn velocity_history_z(&self) -> f32 {
        self.velocity_history.iter()
            .map(|v| v.z)
            .sum::<f32>() / self.velocity_history.len() as f32
    }

    #[func]
    fn ready(&mut self) {
        self.init_raycasts();
        self.reset_state();
    }

    fn init_raycasts(&mut self) {
        let angle_step = FOV / (RAYCAST_NUM - 1) as f32;
        for i in 0..RAYCAST_NUM {
            let mut ray = RayCast3D::new_alloc();
            let angle = -FOV/2.0 + angle_step * i as f32;
            
            ray.set_rotation(Vector3::new(0.0, angle, 0.0));
            ray.set_target_position(Vector3::FORWARD * MAX_DETECTION_RANGE);
            ray.set_collision_mask(0b1);
            ray.set_enabled(true);
            
            self.base_mut().add_child(&ray.clone().upcast::<RayCast3D>());
            self.raycasts.push(ray);
        }
    }

    #[func]
    fn update_agent(&mut self, delta: f64) {
        let delta = delta as f32;
        let player_info = self.detect_player();
        let mut reward = 0.0;

        // Handle player detection
        if player_info.found {
            if !self.player_visible {
                reward += REWARD_FOUND_PLAYER;
            }
            self.last_player_pos = player_info.position;
            self.player_visible = true;
            reward += REWARD_MAINTAIN_LOS;
        } else if self.player_visible {
            reward += PUNISH_LOST_PLAYER;
            self.player_visible = false;
        }

        // Create input vector
        let input = self.create_input_vector(player_info);
        let output = self.forward(&input);
        
        // Convert output to movement
        let move_dir = Vector3::new(output[0], 0.0, output[1]).normalized();
        let velocity = move_dir * AGENT_SPEED;
        
        // Apply movement
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        
        // Distance-based rewards
        if self.player_visible {
            let new_dist = self.base().get_global_position().distance_to(self.last_player_pos);
            if new_dist < self.last_distance {
                reward += REWARD_CLOSER * (self.last_distance - new_dist);
            }
            self.last_distance = new_dist;
        }

        // Check for stuck
        if self.is_stuck() {
            reward += PUNISH_STUCK;
            self.apply_escape_move();
        }
        self.velocity_history.push(velocity);
        if self.velocity_history.len() > 10 {
            self.velocity_history.remove(0);
        }

        // Update network
        self.update_network(&input, reward);
    }

    fn detect_player(&mut self) -> PlayerDetection {
        for ray in &self.raycasts {
            if ray.is_colliding() {
                let collider = ray.get_collider().unwrap();
                let node = collider.clone().cast::<CharacterBody3D>();
                
                if node.is_in_group("players") {
                    let position = ray.get_collision_point();
                    return PlayerDetection {
                        found: true,
                        position,
                        distance: position.distance_to(self.base().get_global_position()),
                    };
                }
            }
        }
        PlayerDetection::none()
    }

    fn create_input_vector(&self, detection: PlayerDetection) -> Array1<f32> {
        let pos = self.base().get_global_position();
        let mut input = Array1::zeros(INPUT_SIZE);
        
        // Player relative position
        input[0] = if detection.found { (detection.position.x - pos.x) / MAX_DETECTION_RANGE } else { 0.0 };
        input[1] = if detection.found { (detection.position.z - pos.z) / MAX_DETECTION_RANGE } else { 0.0 };
        input[2] = detection.distance / MAX_DETECTION_RANGE;
        
        // Agent's own rotation
        input[3] = self.rotation_angle.sin();
        input[4] = self.rotation_angle.cos();
        
        // Historical movement
        input[5] = self.velocity_history_x();
        input[6] = self.velocity_history_z();
        
        // Environment awareness
        input[7] = pos.x / 50.0;  // Normalized map position
        input[8] = pos.z / 50.0;
        input[9] = (pos.y - 1.0) / 5.0;  // Height awareness
        
        // Player visibility
        input[10] = if detection.found { 1.0 } else { -1.0 };
        
        input
    }

    fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        let hidden = input.dot(&self.weights_input_hidden).mapv(|x| x.max(0.0));
        let output = hidden.dot(&self.weights_hidden_output).mapv(|x| x.tanh());
        output
    }

    fn update_network(&mut self, input: &Array1<f32>, reward: f32) {
        let hidden = input.dot(&self.weights_input_hidden).mapv(|x| x.max(0.0));
        
        let learning_rate = 0.01;
        let discount_factor = 0.9;
        
        // Clone hidden before first use
        let hidden_clone = hidden.clone();
        let mut q_values = hidden_clone.dot(&self.weights_hidden_output);
        
        let max_future_q = q_values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let update = (reward + discount_factor * max_future_q - q_values.sum()) * learning_rate;
        
        // Apply update to all elements
        q_values.mapv_inplace(|x| x + update);
        
        // Create views with cloning for axis operations
        let hidden_t = hidden.clone().insert_axis(ndarray::Axis(1));
        let input_t = input.clone().insert_axis(ndarray::Axis(1));
        
        // Update weights using cloned arrays
        let delta_output = hidden_t.dot(&q_values.clone().insert_axis(ndarray::Axis(0)));
        self.weights_hidden_output += &(delta_output * learning_rate);
        
        let delta_hidden = input_t.dot(&hidden.insert_axis(ndarray::Axis(0)));
        self.weights_input_hidden += &(delta_hidden * learning_rate);
    }

    fn is_stuck(&self) -> bool {
        if self.positions_history.len() < 10 { return false; }
        let avg = self.positions_history.iter()
            .fold(Vector3::ZERO, |acc, pos| acc + *pos) 
            / self.positions_history.len() as f32;
        avg.distance_to(*self.positions_history.last().unwrap()) < 0.5
    }

    fn apply_escape_move(&mut self) {
        let escape_dir = Vector3::new(
            rand::rng().random_range(-1.0..1.0),
            0.0,
            rand::rng().random_range(-1.0..1.0)
        ).normalized();
        self.base_mut().set_velocity(escape_dir * AGENT_SPEED * 2.0);
    }

    fn reset_state(&mut self) {
        self.last_player_pos = Vector3::ZERO;
        self.player_visible = false;
        self.last_distance = f32::MAX;
        self.episode_reward = 0.0;
        self.positions_history.clear();
        self.base_mut().set_position(Vector3::new(
            rand::rng().random_range(-20.0..20.0),
            1.0,
            rand::rng().random_range(-20.0..20.0)
        ));
    }
}

// Helper structs
#[derive(Debug, Clone, Copy)]
struct PlayerDetection {
    found: bool,
    position: Vector3,
    distance: f32,
}

impl PlayerDetection {
    fn none() -> Self {
        Self {
            found: false,
            position: Vector3::ZERO,
            distance: f32::MAX,
        }
    }
}