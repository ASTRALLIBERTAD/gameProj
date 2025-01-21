extends Rustplayer

func _ready() -> void:
	SaveManager.player_node = self
	if is_instance_valid(SaveManager.player_node):
		emit_signal("player_ready")
	else:
		print("Error: player_node is invalid")
