use godot::prelude::*;
use godot::classes::Node;
use tch::{nn::{self, Module, VarStore}, Device, Kind, Tensor};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GodotRLAgent {
    #[base]
    base: Base<Node>,
    
    vs: VarStore,
    agent: Option<RLAgent>,
}

#[godot_api]
impl INode for GodotRLAgent {
    fn init(base: Base<Node>) -> Self {
        let vs = VarStore::new(Device::Cpu);
        let mut agent_struct = Self {
            base,
            vs,
            agent: None,
        };
        agent_struct.agent = Some(RLAgent::new(&agent_struct.vs.root()));
        agent_struct
    }
}

#[godot_api]
impl GodotRLAgent {
    #[func]
    fn choose_action(&mut self, state: Variant) -> i64 {
        let state_vec: Vec<f32> = state
            .to::<PackedFloat32Array>()
            .to_vec();
        
        let state_tensor = Tensor::f_from_slice(&state_vec)
            .unwrap()
            .reshape(&[1, -1])
            .to_kind(Kind::Float);
        
        self.agent
            .as_mut()
            .expect("Agent not initialized")
            .choose_action(&state_tensor)
    }

    #[func]
    fn update(&mut self, state: Variant, action: i64, reward: f64, next_state: Variant) {
        let state_vec: Vec<f32> = state.to::<PackedFloat32Array>().to_vec();
        let next_state_vec: Vec<f32> = next_state.to::<PackedFloat32Array>().to_vec();
        
        let state_tensor = Tensor::f_from_slice(&state_vec).unwrap().reshape(&[1, -1]).to_kind(Kind::Float);
        let next_state_tensor = Tensor::f_from_slice(&next_state_vec).unwrap().reshape(&[1, -1]).to_kind(Kind::Float);
        
        if let Some(agent) = self.agent.as_mut() {
            agent.store_experience(state_tensor, action, reward, next_state_tensor);
            agent.train();
        } else {
            godot_error!("Agent not initialized");
        }
    }
    
    #[func]
    fn save_model(&self, path: GString) {
        if let Err(e) = self.vs.save(path.to_string()) {
            godot_error!("Failed to save model: {}", e);
        }
    }

    #[func]
    fn load_model(&mut self, path: GString) {
        if let Err(e) = self.vs.load(path.to_string()) {
            godot_error!("Failed to load model: {}", e);
        }
    }
}

struct RLAgent {
    model: nn::Sequential,
}

impl RLAgent {
    fn new(vs: &nn::Path) -> Self {
        let model = nn::seq()
            .add(nn::linear(vs, 4, 16, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs, 16, 2, Default::default()));
        
        Self { model }
    }

    fn choose_action(&mut self, state: &Tensor) -> i64 {
        let output = self.model.forward(state);
        let (_, action) = output.max_dim(1, false);
        i64::from(action.int64_value(&[]))
    }

    fn store_experience(&mut self, _state: Tensor, _action: i64, _reward: f64, _next_state: Tensor) {
        // Placeholder: Implement experience replay buffer
    }

    fn train(&mut self) {
        // Placeholder: Implement training logic
    }
}
