use godot::classes::{INode, Node, PacketPeerUdp};
use godot::prelude::*;

const LISTEN_PORT: i32 = 8912;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MultiplayerScene {
    base: Base<Node>,
    listener: Gd<PacketPeerUdp>
}

#[godot_api]
impl INode for MultiplayerScene {
    fn init(base: Base<Node>) -> Self {
        Self { 
            base,
            listener: PacketPeerUdp::new_gd()
        }
    }

    fn ready(&mut self) {
        self.set_up();

    }

    fn physics_process(&mut self, delta: f64) {
        while self.listener.get_available_packet_count() > 0 {
            let serverip = self.listener.get_packet_ip();
            let serverport = self.listener.get_packet_port();
            let packet = self.listener.get_packet();
            
            //Convert bytes to string using get_string_from_utf8() instead of get_string_from_ascii()
            let data = packet.get_string_from_ascii();

        } {
            
        }

    }
}

#[godot_api]
impl MultiplayerScene {
    
    #[func]
    fn set_up(&mut self) {
        self.listener.bind(LISTEN_PORT);
    }

}