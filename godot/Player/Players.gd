extends Rustplayer

func _ready() -> void:
	SaveManager.player_node = self

func _on_timer_timeout() -> void:
	$Camera2D.position_smoothing_enabled = true
	$Camera2D.set_position_smoothing_enabled(2)
	pass # Replace with function body.

func _on_inventory_pressed() -> void:
	open_close()

func full_or_not():
	var is_full = true
	for i in inv.slots:
		if i.item.name == "":
			is_full = false
			return is_full
	return is_full


func player():
	pass
