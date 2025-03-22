extends Rustplayer

var is_open = false


func _ready() -> void:
	SaveManager.player_node = self

func _on_timer_timeout() -> void:
	$Camera2D.position_smoothing_enabled = true
	$Camera2D.set_position_smoothing_enabled(2)
	pass # Replace with function body.

func _on_inventory_pressed() -> void:
	if is_open:
		close()
	else:
		open()
	pass # Replace with function body.

func open():
	%ItemSlot.visible = true
	is_open = true

func close():
	%ItemSlot.visible = false
	is_open = false
