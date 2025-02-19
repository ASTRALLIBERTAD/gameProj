extends Node

var peer = ENetMultiplayerPeer.new()
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_join_pressed() -> void:
	peer.create_client("localhost", 55555)
	multiplayer.multiplayer_peer = peer
	var p =get_node("/root/main/World")
	var t = preload("uid://cb7g0u1n88g4v").instantiate()
	add_child(t)
	pass # Replace with function body.
