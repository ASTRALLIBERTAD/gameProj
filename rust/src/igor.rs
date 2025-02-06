use godot::prelude::*;
use godot::classes::{CharacterBody3D, RayCast3D};
use rand::Rng;
use ndarray::{Array1, Array2, ArrayView1};
use std::collections::VecDeque;

const INPUT_SIZE: usize = 11;
const HIDDEN_SIZE: usize = 32;
const OUTPUT_SIZE: usize = 6;  // Discrete actions: +x, -x, +y, -y, +z, -z
const REPLAY_BUFFER_SIZE: usize = 10000;
const BATCH_SIZE: usize = 32;
const GAMMA: f32 = 0.99;
const EPSILON_START: f32 = 1.0;
const EPSILON_END: f32 = 0.01;
const EPSILON_DECAY: f32 = 0.995;
const TARGET_UPDATE_FREQ: usize = 100;
const LEARNING_RATE: f32 = 0.001;

#[derive(Clone)]
struct Experience {
    state: Array1<f32>,
    action: usize,
    reward: f32,
    next_state: Array1<f32>,
    done: bool,
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct DQNAgent {
    #[base]
    base: Base<CharacterBody3D>,
    
    // Main network
    weights1: Array2<f32>,
    weights2: Array2<f32>,
    
    // Target network
    target_weights1: Array2<f32>,
    target_weights2: Array2<f32>,
    
    replay_buffer: VecDeque<Experience>,
    epsilon: f32,
    steps: usize,
    timer: f32,
    max_time: f32,
}

#[godot_api]
impl DQNAgent {
    #[func]
    fn init(&mut self) {
        let mut rng = rand::rng();
        
        // Initialize main network
        self.weights1 = Array2::from_shape_fn((INPUT_SIZE, HIDDEN_SIZE), 
            |_| rng.random_range(-0.1..0.1));
        self.weights2 = Array2::from_shape_fn((HIDDEN_SIZE, OUTPUT_SIZE), 
            |_| rng.random_range(-0.1..0.1));
            
        // Initialize target network
        self.target_weights1 = self.weights1.clone();
        self.target_weights2 = self.weights2.clone();
        
        self.replay_buffer = VecDeque::with_capacity(REPLAY_BUFFER_SIZE);
        self.epsilon = EPSILON_START;
        self.steps = 0;
        self.timer = 0.0;
        self.max_time = 10.0;
    }

    #[func]
    fn update_movement(&mut self, raycast: Gd<RayCast3D>, player: Gd<CharacterBody3D>, delta: f64) {
        self.timer += delta as f32;
        self.steps += 1;

        // Get current state
        let state = self.get_state(&raycast, &player);
        
        // Choose action using epsilon-greedy policy
        let action = self.select_action(&state);
        
        // Convert discrete action to movement vector
        let movement_vector = self.action_to_vector(action);
        
        // Apply movement
        self.base_mut().set_velocity(movement_vector * 2.0);
        self.base_mut().move_and_slide();
        
        // Get new state and calculate reward
        let next_state = self.get_state(&raycast, &player);
        let (reward, done) = self.calculate_reward(&player);
        
        // Store experience
        self.store_experience(Experience {
            state: state.clone(),
            action,
            reward,
            next_state: next_state.clone(),
            done,
        });
        
        // Train network
        if self.replay_buffer.len() >= BATCH_SIZE {
            self.train_network();
        }
        
        // Update target network
        if self.steps % TARGET_UPDATE_FREQ == 0 {
            self.update_target_network();
        }
        
        // Update epsilon
        self.epsilon = (EPSILON_END + (EPSILON_START - EPSILON_END) * 
            (-self.steps as f32 / 1000.0).exp()).max(EPSILON_END);
            
        // Reset if done
        if done {
            self.reset_agent();
        }
    }
    
    fn get_state(&self, raycast: &Gd<RayCast3D>, player: &Gd<CharacterBody3D>) -> Array1<f32> {
        let npc_position = self.base_mut().get_global_position();
        let player_position = player.get_global_position();
        let hit_position = raycast.get_collision_point();
        
        Array1::from(vec![
            player_position.x, player_position.y, player_position.z,
            hit_position.x, hit_position.y, hit_position.z,
            (player_position - hit_position).length(),
            npc_position.x, npc_position.y, npc_position.z,
            1.0,
        ])
    }
    
    fn select_action(&self, state: &Array1<f32>) -> usize {
        let mut rng = rand::rng();
        
        if rng.random::<f32>() < self.epsilon {
            rng.random_range(0..OUTPUT_SIZE)
        } else {
            let q_values = self.forward(state, &self.weights1, &self.weights2);
            q_values.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(index, _)| index)
                .unwrap()
        }
    }
    
    fn action_to_vector(&self, action: usize) -> Vector3 {
        match action {
            0 => Vector3::new(1.0, 0.0, 0.0),   // +x
            1 => Vector3::new(-1.0, 0.0, 0.0),  // -x
            2 => Vector3::new(0.0, 0.5, 0.0),   // +y (limited vertical movement)
            3 => Vector3::new(0.0, -0.5, 0.0),  // -y
            4 => Vector3::new(0.0, 0.0, 1.0),   // +z
            5 => Vector3::new(0.0, 0.0, -1.0),  // -z
            _ => Vector3::ZERO,
        }
    }
    
    fn calculate_reward(&self, player: &Gd<CharacterBody3D>) -> (f32, bool) {
        let distance = (self.base_mut().get_global_position() - 
            player.get_global_position()).length();
            
        let mut reward = -distance * 0.1;  // Small negative reward based on distance
        let mut done = false;
        
        if distance < 1.0 {
            reward += 10.0;  // Big reward for reaching target
            done = true;
        }
        
        if self.timer > self.max_time {
            reward -= 5.0;  // Penalty for timeout
            done = true;
        }
        
        (reward, done)
    }
    
    fn store_experience(&mut self, experience: Experience) {
        if self.replay_buffer.len() >= REPLAY_BUFFER_SIZE {
            self.replay_buffer.pop_front();
        }
        self.replay_buffer.push_back(experience);
    }
    
    fn train_network(&mut self) {
        let mut rng = rand::rng();
        let batch: Vec<_> = (0..BATCH_SIZE)
            .map(|_| {
                let idx = rng.random_range(0..self.replay_buffer.len());
                self.replay_buffer[idx].clone()
            })
            .collect();
            
        // Calculate target Q-values
        let mut total_loss = 0.0;
        
        for experience in batch {
            let q_values = self.forward(&experience.state, &self.weights1, &self.weights2);
            let next_q_values = self.forward(&experience.next_state, 
                &self.target_weights1, &self.target_weights2);
            
            let target = if experience.done {
                experience.reward
            } else {
                experience.reward + GAMMA * next_q_values.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))
            };
            
            // Calculate loss and gradients
            let mut new_q_values = q_values.clone();
            new_q_values[experience.action] = target;
            
            // Perform gradient descent
            self.backward(&experience.state, &q_values, &new_q_values);
            
            total_loss += (target - q_values[experience.action]).powi(2);
        }
        
        godot_print!("Loss: {}", total_loss / BATCH_SIZE as f32);
    }
    
    fn forward(&self, input: &Array1<f32>, weights1: &Array2<f32>, weights2: &Array2<f32>) -> Array1<f32> {
        let hidden = input.dot(weights1).mapv(|x| x.max(0.0));  // ReLU
        hidden.dot(weights2)
    }
    
    fn backward(&mut self, state: &Array1<f32>, predicted: &Array1<f32>, target: &Array1<f32>) {
        // Simple gradient descent implementation
        let hidden = state.dot(&self.weights1).mapv(|x| x.max(0.0));
        let delta_output = target - predicted;
        
        // Update weights2
        let delta_hidden = delta_output.dot(&self.weights2.t()) * 
            hidden.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 });
            
        self.weights2 += &(LEARNING_RATE * hidden.view().insert_axis(ndarray::Axis(1))
            .dot(&delta_output.view().insert_axis(ndarray::Axis(0))));
            
        // Update weights1
        self.weights1 += &(LEARNING_RATE * state.view().insert_axis(ndarray::Axis(1))
            .dot(&delta_hidden.view().insert_axis(ndarray::Axis(0))));
    }
    
    fn update_target_network(&mut self) {
        self.target_weights1 = self.weights1.clone();
        self.target_weights2 = self.weights2.clone();
    }
    
    fn reset_agent(&mut self) {
        self.base_mut().set_global_position(Vector3::new(0.0, 1.0, 0.0));
        self.timer = 0.0;
    }
}