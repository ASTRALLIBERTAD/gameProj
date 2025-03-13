extends Rustplayer

func _ready() -> void:
	SaveManager.player_node = self

func _on_timer_timeout() -> void:
	$Camera2D.position_smoothing_enabled = true
	$Camera2D.set_position_smoothing_enabled(2)
	pass # Replace with function body.
