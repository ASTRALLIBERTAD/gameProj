extends Rustplayer

func _ready() -> void:
	SaveManager.player_node = self 
	emit_signal("player_ready")
	
