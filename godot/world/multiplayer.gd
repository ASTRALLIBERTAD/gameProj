extends Node

const BROADCAST_PORT = 55555
var udp := PacketPeerUDP.new()
var found_servers := []

func _ready():
	udp.bind(BROADCAST_PORT)
	print("Client ready to scan for servers.")

func scan_for_servers():
	found_servers.clear()
	print("Scanning for servers...")

func _process(delta):
	if udp.get_available_packet_count() > 0:
		var packet = udp.get_packet().get_string_from_utf8()
		if packet.begins_with("GODOT_SERVER|"):
			var server_ip = packet.replace("GODOT_SERVER|", "")
			if server_ip not in found_servers:
				found_servers.append(server_ip)
				print("Found server at:", server_ip)

func _on_join_pressed(ip):
	var peer = ENetMultiplayerPeer.new()
	peer.create_client(ip, 55555)
	multiplayer.multiplayer_peer = peer
	print("Connecting to:", ip)
	get_tree().change_scene_to_file("res://World.tscn")
