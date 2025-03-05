use std::str;
use godot::classes::{ INode2D, Node2D, PacketPeerUdp};
use godot::obj::NewGd;
use godot::prelude::*;

const BROADCAST_PORT: i32 = 8912;
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Node2dRust {
    #[base]
    base: Base<Node2D>,
    udp: Gd<PacketPeerUdp>

}

#[godot_api]
impl INode2D for Node2dRust {
    fn init(base: Base<Node2D>) -> Self {
        Self 
        { 
            base,
            udp: PacketPeerUdp::new_gd(),
        }
    }
    fn ready(&mut self) {
    }

}

#[godot_api]
impl Node2dRust {
    
    #[func]
    fn broadcast(&mut self) {
        self.udp.set_broadcast_enabled(true);
        self.udp.bind(BROADCAST_PORT);

    }

    #[func]
    fn broadcaster_timeout(&mut self, packet: PackedByteArray) {
        self.udp.put_packet(&packet);
        self.udp.set_dest_address("255.255.255.255", BROADCAST_PORT);
        godot_print!("{}", packet)
    }
}



