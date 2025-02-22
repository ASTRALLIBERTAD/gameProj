extends Node

var peer = ENetMultiplayerPeer.new()


func _on_join_pressed():
	peer.create_client("localhost", 55555)  
	multiplayer.multiplayer_peer = peer
	get_tree().change_scene_to_file("res://World.tscn")
	print("🌍 Trying to connect to server...")
