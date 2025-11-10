use std::str;
use godot::classes::{ INode2D, Node2D, PacketPeerUdp};
use godot::obj::NewGd;
use godot::prelude::*;

use crate::rustplayer::Rustplayer;

const BROADCAST_PORT: i32 = 8912;
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Node2dRust {
    #[base]
    base: Base<Node2D>,
    udp: Gd<PacketPeerUdp>,
    #[export]
    authority_player: OnEditor<Gd<Rustplayer>>,
    #[export]
    pub player_node_names: Array<GString>,

}

#[godot_api]
impl INode2D for Node2dRust {
    fn init(base: Base<Node2D>) -> Self {
        Self 
        { 
            base,
            udp: PacketPeerUdp::new_gd(),
            authority_player: OnEditor::default(),
            player_node_names: Array::default(),
        }
    }
    fn ready(&mut self) {

        // let unique_id = self.base_mut().get_multiplayer().unwrap().get_unique_id();
        // self.authority_player.set_multiplayer_authority(unique_id);
    }

}

#[godot_api]
impl Node2dRust {
    
    #[func]
    fn broadcast(&mut self) {
        self.udp.set_broadcast_enabled(true);
        self.udp.bind(BROADCAST_PORT);
        godot_print!("UDP Broadcaster started on port {}", BROADCAST_PORT);

    }

    #[func]
    fn broadcaster_timeout(&mut self, packet: PackedByteArray) {
        godot_print!("Broadcasting server info...");
        
        self.udp.set_dest_address("255.255.255.255", BROADCAST_PORT);
        
        self.udp.put_packet(&packet);
        
        godot_print!("{}", packet)
    }
}



