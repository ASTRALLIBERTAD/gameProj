use godot::classes::{ENetMultiplayerPeer, HBoxContainer, INode, Json, Label, Node, PacketPeerUdp, VBoxContainer};
use godot::prelude::*;

const LISTEN_PORT: i32 = 8912;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MultiplayerScene {
    base: Base<Node>,
    listener: Gd<PacketPeerUdp>,
    #[export]
    player: OnEditor<Gd<PackedScene>>,

    #[export]
    server_info: OnEditor<Gd<PackedScene>>,
}

#[godot_api]
impl INode for MultiplayerScene {
    fn init(base: Base<Node>) -> Self {
        Self { 
            base,
            listener: PacketPeerUdp::new_gd(),
            player: OnEditor::default(),
            server_info: OnEditor::default()
        }
    }

    fn ready(&mut self) {
        self.set_up();

    }

    fn exit_tree(&mut self) {
        self.clean_up();
    }

    fn process(&mut self, _delta: f64) {
        while self.listener.get_available_packet_count() > 0 {
            let serverip = self.listener.get_packet_ip();
            let serverport = self.listener.get_packet_port();
            let packet = self.listener.get_packet();

            
            let data = packet.get_string_from_ascii();

            let mut json = Json::new_gd();
            let parse_result = json.parse(&data);

            if parse_result == godot::global::Error::OK {
                let room_info = json.get_data();
                let dict = room_info.to::<Dictionary>();

                godot_print!(
                    "Server IP: {} Server Port: {} Room info: {:?}",
                    serverip,
                    serverport,
                    dict
                );

                let name_str = dict
                    .get("name")
                    .unwrap_or(Variant::from("Unnamed"))
                    .to::<GString>();

                let mut vbox = self
                    .base_mut()
                    .get_node_as::<VBoxContainer>("CanvasLayer/Panel/VBoxContainer");

                
                let mut room_exist = false;

                for i in vbox.get_children().iter_shared() {
                    if i.get_name().to_string() == name_str.to_string() {
                        
                        let ip_label = i.try_get_node_as::<Label>("Ip");
                        if let Some(mut ip_label) = ip_label {
                            ip_label.set_text(&serverip);
                        }
                        room_exist = true;
                        break;
                    }
                }

                
                if !room_exist {
                    let mut current_info = self.server_info.instantiate_as::<HBoxContainer>();

                    
                    current_info.set_name(&name_str);

                    // Access children relative to the instance itself
                    if let Some(mut ip_label) = current_info.try_get_node_as::<Label>("Ip") {
                        ip_label.set_text(&serverip);
                    }

                    if let Some(mut name_label) = current_info.try_get_node_as::<Label>("Name") {
                        name_label.set_text(&name_str);
                    }

                    // Add it to the server list
                    vbox.add_child(&current_info);

                    // Connect signal
                    let callable = self.base_mut().callable("joinby_ip");
                    current_info.connect("joinGame", &callable);

                    godot_print!("Added new server: {}", name_str);
                }
            }
        }
}



}

#[godot_api]
impl MultiplayerScene {
    
    #[func]
    fn set_up(&mut self) {
        let mut listener = PacketPeerUdp::new_gd();

        let result = listener.bind(LISTEN_PORT);
        if result == godot::global::Error::OK {
            godot_print!("Successfully bound to port: {}", LISTEN_PORT);
        } else {
            godot_error!("Failed to bind to port: {}", LISTEN_PORT);
            return;
        }

        listener.set_broadcast_enabled(true);
   
        self.listener = listener;
    }

    #[func]
    fn joinby_ip(&mut self, ip: GString) {
        self.base_mut().emit_signal("join_game", &[ip.to_variant()]);
        
    }

    #[func]
    fn d(&mut self, ip: GString) {
        let mut peer = ENetMultiplayerPeer::new_gd();
        let error = peer.create_client(&ip.to_string(), 55555);
        
        if error == godot::global::Error::OK {
            self.base_mut().get_multiplayer().unwrap().set_multiplayer_peer(&peer);
            godot_print!("Connecting {}", ip);
            self.base_mut().get_tree().unwrap().change_scene_to_file("res://World.scn");
        } else {
            godot_print!("Failed to create client. Error code: {:?}", error);
        }
    }

    #[func]
    fn on_back_pressed(&mut self) {
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://SaveAndLoad/LoadMenu.scn");
    }

    #[func]
    fn clean_up(&mut self) {
        self.listener.close();
    }

}
