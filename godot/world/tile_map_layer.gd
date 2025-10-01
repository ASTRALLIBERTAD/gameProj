extends TileMapLayer


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if  Input.is_action_just_pressed("down"):
		var mouse_pos = get_global_mouse_position()
		var local = local_to_map(mouse_pos)
		set_cells_terrain_connect([local],0,0)
	pass
