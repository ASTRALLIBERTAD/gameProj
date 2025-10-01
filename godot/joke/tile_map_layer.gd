extends TileMapLayer


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var mouse_pos = get_global_mouse_position()
	var local = local_to_map(mouse_pos)
	if  Input.is_action_pressed("click"):
		set_cells_terrain_connect([local],0,1, true)
	pass
