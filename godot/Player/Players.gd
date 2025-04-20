extends Rustplayer

func _ready() -> void:
	SaveManager.player_node = self

func _on_timer_timeout() -> void:
	$Camera2D.position_smoothing_enabled = true
	$Camera2D.set_position_smoothing_enabled(2)
	pass # Replace with function body.

func _on_inventory_pressed() -> void:
	open_close()

func full_or_not() -> bool:
	var is_full = true
	for i in inv.slots:
		if i.item.name == "":
			is_full = false
			return is_full
	return is_full

func item_stckable() -> bool:
	var is_stackable = false
	for slots in inv.slots:
		if slots.item.stackable == true:
			if slots.item.name == slots.item.name:
				is_stackable = true
				return is_stackable
	return is_stackable

func get_item_name() -> String:
	var item_name = ""
	for get_name in inv.slots:
		var t = get_name.item.name
		item_name = t
		return item_name
	return item_name

func player():
	pass
